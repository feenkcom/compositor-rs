use anyhow::{Result, anyhow, bail};
use mozangle::egl::ffi::*;
use mozangle::egl::{ffi, get_proc_address};
use skia_safe::gpu::gl::{Format, FramebufferInfo, Interface};
use skia_safe::gpu::{
    BackendRenderTarget, ContextOptions, DirectContext, RecordingContext, SurfaceOrigin,
};
use skia_safe::{ColorType, ColorSpace, ISize, Surface, gpu};
use std::ffi::{CString, c_void};
use std::fmt::{Debug, Formatter};
use std::mem::transmute;
use std::os::raw;
use windows::core::{Error, PCWSTR, PWSTR, BOOL};
use windows::Win32::Foundation::{HWND, MAX_PATH};
use windows::Win32::Graphics::Gdi::{
    GetDC,CreateDCW, DeleteDC, GetMonitorInfoW, MonitorFromWindow,
    MONITORINFOEXW, MONITOR_DEFAULTTONEAREST, HDC
};
use windows::Win32::UI::ColorSystem::GetICMProfileW;

use crate::OpenGLPlatform;
use crate::angle_utils::*;

pub const SAMPLE_COUNT: u32 = 1;

#[derive(Debug)]
pub struct AngleContext {
    window: HWND,
    egl_display: types::EGLDisplay,
    egl_context: Option<AngleWindowContext>,
    width: i32,
    height: i32,
    recreate_context_on_resize: bool,
}

pub unsafe extern "C" fn get_proc_address_ffi(name: *const raw::c_char) -> *const c_void {
    GetProcAddress(name) as *const _ as _
}

pub unsafe extern "C" fn get_current_context_ffi() -> *const c_void {
    GetCurrentContext()
}

fn get_monitor_icc_profile(hwnd: HWND) -> windows::core::Result<Option<Vec<u8>>> {
    unsafe {
        let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let mut monitor_info: MONITORINFOEXW = std::mem::zeroed();
        monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

        let ok = GetMonitorInfoW(hmonitor, &mut monitor_info.monitorInfo as *mut _);
        if !ok.as_bool() {
            return Err(Error::from_thread());
        }

        let monitor_device_context = CreateDCW(
            PCWSTR::null(),
            PCWSTR(monitor_info.szDevice.as_ptr()),
            PCWSTR::null(),
            None,
        );
        
        if monitor_device_context.is_invalid() {
            return Err(Error::from_thread());
        }

        // Ask GDI for the ICC profile path bound to this device context
        let mut buffer: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let mut buffer_size: u32 = buffer.len() as u32;

        let profile_path_result = GetICMProfileW(monitor_device_context, &mut buffer_size, Some(PWSTR(buffer.as_mut_ptr())));

        DeleteDC(monitor_device_context);

        if !profile_path_result.as_bool() {
            return Err(Error::from_thread());
        }

        let profile_path = String::from_utf16_lossy(&buffer[..buffer_size as usize])
            .trim_end_matches('\0')
            .to_string();

        if profile_path.is_empty() {
            return Ok(None);
        }

        let profile_bytes = std::fs::read(&profile_path)?;

        Ok(Some(profile_bytes))
    }
}

impl AngleContext {
    pub fn platform(&self) -> Option<OpenGLPlatform> {
        self.egl_context
            .as_ref()
            .map(|window_context| OpenGLPlatform {
                display: self.egl_display as *mut c_void,
                context: window_context.egl_context as *mut c_void,
                surface: window_context.egl_surface as *mut c_void,
                get_proc_address: get_proc_address_ffi,
                get_current_context: get_current_context_ffi,
            })
    }

    pub fn new(
        window: *mut c_void,
        width: i32,
        height: i32,
        recreate_context_on_resize: bool,
    ) -> Result<Self> {
        let window: HWND = unsafe { transmute(window) };

        let (egl_display, _major_version, _minor_version) = get_display(window)?;

        let mut angle_context = Self {
            window,
            egl_display,
            egl_context: None,
            width,
            height,
            recreate_context_on_resize,
        };

        angle_context.initialize_context()?;

        info!("Initialized Angle context {:?}", &angle_context);

        Ok(angle_context)
    }

    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) -> Result<()> {
        match self.make_current() {
            Ok(_) => {}
            Err(error) => {
                warn!("Failed to make context current: {:?}", error);
                let _ = self.destroy_context();

                let (egl_display, _major_version, _minor_version) = get_display(self.window)?;
                self.egl_display = egl_display;
                self.initialize_context()?;
                self.make_current()?;
            }
        }

        if let Some(surface) = self.get_surface() {
            trace!(
                "About to draw on a surface of size {}x{}",
                surface.width(),
                surface.height()
            );
            callback(surface);
            self.flush_and_submit();
        }
        self.swap_buffers()?;

        Ok(())
    }

    pub fn resize_surface(&mut self, size: ISize) -> Result<()> {
        debug!(
            "About to resize angle context to {}x{}",
            size.width, size.height
        );
        self.width = size.width;
        self.height = size.height;

        if self.recreate_context_on_resize {
            self.destroy_context()?;
            self.initialize_context()?;
        }

        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.try_create_surface(self.width, self.height)?;
        }

        Ok(())
    }

    fn get_surface(&mut self) -> Option<&mut Surface> {
        if let Some(ref mut egl_context) = self.egl_context {
            if egl_context.skia_surface.is_none() {
                match egl_context.try_create_surface(self.width, self.height) {
                    Ok(_) => {}
                    Err(error) => {
                        error!("Failed to initialize surface: {:?}", error);
                    }
                };
            }
            return egl_context.skia_surface.as_mut();
        }
        None
    }

    fn initialize_context(&mut self) -> Result<()> {
        if self.egl_context.is_some() {
            bail!("Context already initialized")
        }

        let surface_size = if self.recreate_context_on_resize {
            Some((self.width, self.height))
        } else {
            None
        };

        self.egl_context = Some(AngleWindowContext::try_create(
            self.egl_display,
            self.window,
            surface_size,
        )?);
        Ok(())
    }

    fn destroy_context(&mut self) -> Result<()> {
        if let Some(mut egl_context) = self.egl_context.take() {
            egl_context.destroy_context()?;
        }
        Ok(())
    }

    fn make_current(&mut self) -> Result<()> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.make_current()?;
        }
        Ok(())
    }

    fn make_not_current(&mut self) -> Result<()> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.make_not_current()?;
        }
        Ok(())
    }

    fn swap_buffers(&mut self) -> Result<()> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.swap_buffers()?;
        }
        Ok(())
    }

    fn flush_and_submit(&mut self) {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.direct_context.flush_and_submit();
        }
    }
}

impl Drop for AngleContext {
    fn drop(&mut self) {
        self.destroy_context()
            .unwrap_or_else(|error| error!("{:?}", error));
        terminate_display(self.egl_display).unwrap_or_else(|error| error!("{:?}", error));
        self.egl_display = NO_DISPLAY;
    }
}

pub struct AngleWindowContext {
    egl_display: types::EGLDisplay,
    egl_config: types::EGLConfig,
    egl_context: types::EGLContext,
    egl_surface: types::EGLSurface,
    backend_context: Interface,
    direct_context: DirectContext,
    skia_surface: Option<Surface>,
    color_space: Option<ColorSpace>,
}

impl AngleWindowContext {
    fn try_create(
        egl_display: types::EGLDisplay,
        window: HWND,
        size: Option<(i32, i32)>,
    ) -> Result<Self> {
        let egl_config = choose_config(egl_display)?;
        let egl_context = create_context(egl_display, egl_config)?;

        let egl_surface = create_window_surface(egl_display, egl_config, window, size)?;
        make_current(egl_display, egl_surface, egl_surface, egl_context)?;
        let interface = assemble_interface()?;
        let context_options = ContextOptions::default();
        let direct_context = gpu::direct_contexts::make_gl(interface.clone(), &context_options)
            .ok_or_else(|| anyhow!("Failed to create direct context"))?;

        let icc_profile = get_monitor_icc_profile(window);
        let color_space = 
            match icc_profile {
                Ok(Some(profile)) => ColorSpace::new_icc(profile.as_slice()),
                _ => None
            };

        Ok(Self {
            egl_display,
            egl_config,
            egl_context,
            egl_surface,
            backend_context: interface,
            direct_context,
            skia_surface: None,
            color_space,
        })
    }

    fn try_create_surface(&mut self, width: i32, height: i32) -> Result<()> {
        debug!(
            "About to create a skia surface of size {}x{}",
            width, height
        );
        let skia_surface = create_skia_surface(&mut self.direct_context, width, height, self.color_space.clone())?;
        self.skia_surface = Some(skia_surface);
        Ok(())
    }

    fn make_current(&self) -> Result<()> {
        make_current(
            self.egl_display,
            self.egl_surface,
            self.egl_surface,
            self.egl_context,
        )
    }

    fn make_not_current(&self) -> Result<()> {
        make_current(self.egl_display, NO_SURFACE, NO_SURFACE, NO_CONTEXT)
    }

    fn swap_buffers(&self) -> Result<()> {
        swap_buffers(self.egl_display, self.egl_surface)
    }

    fn destroy_context(&mut self) -> Result<()> {
        self.make_not_current()?;
        destroy_window_surface(self.egl_display, self.egl_surface)?;
        destroy_egl_context(self.egl_display, self.egl_context)?;
        self.egl_context = NO_CONTEXT;
        self.egl_surface = NO_SURFACE;
        Ok(())
    }
}

impl Debug for AngleWindowContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SkiaContext")
            .field("skia_surface", &self.skia_surface)
            .finish()
    }
}

fn assemble_interface() -> Result<Interface> {
    Interface::new_load_with(|name| get_proc_address(name))
        .ok_or_else(|| anyhow!("Failed to create interface"))
}

fn create_direct_context(interface: Interface) -> Result<DirectContext> {
    gpu::direct_contexts::make_gl(interface.clone(), None)
        .ok_or_else(|| anyhow!("Failed to create direct context"))
}

fn create_skia_surface(
    recording_context: &mut RecordingContext,
    width: i32,
    height: i32,
    color_space: Option<ColorSpace>
) -> Result<Surface> {
    let framebuffer = get_framebuffer_binding();

    let framebuffer_info = FramebufferInfo {
        fboid: framebuffer.try_into()?,
        format: Format::RGBA8.into(),
        protected: gpu::Protected::No,
    };

    let backend_render_target = gpu::backend_render_targets::make_gl(
        (width, height),
        SAMPLE_COUNT as usize,
        0,
        framebuffer_info,
    );

    gpu::surfaces::wrap_backend_render_target(
        recording_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        color_space,
        None,
    )
    .ok_or_else(|| anyhow!("Failed to create skia surface"))
}
