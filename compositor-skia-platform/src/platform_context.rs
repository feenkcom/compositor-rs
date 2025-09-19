use skia_safe::{ISize, Surface};

#[derive(Debug)]
pub enum PlatformContext {
    #[cfg(target_os = "macos")]
    Metal(crate::MetalContext),
    #[cfg(feature = "d3d")]
    D3D(crate::D3D12Context),
    #[cfg(feature = "angle")]
    Angle(crate::AngleContext),
    #[cfg(feature = "x11")]
    XlibGl(crate::XlibGlWindowContext),
    #[cfg(feature = "egl")]
    Egl(crate::EglContext),
    #[cfg(target_os = "emscripten")]
    WebGL(crate::WebGLContext),
    Unsupported,
}

impl PlatformContext {
    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) {
        match self {
            #[cfg(target_os = "macos")]
            PlatformContext::Metal(context) => context.with_surface(callback),
            #[cfg(feature = "d3d")]
            PlatformContext::D3D(context) => context.with_surface(callback),
            #[cfg(feature = "angle")]
            PlatformContext::Angle(context) => context
                .with_surface(callback)
                .unwrap_or_else(|error| error!("Failed to draw on a surface: {}", error)),
            #[cfg(feature = "x11")]
            PlatformContext::XlibGl(context) => context.with_surface(callback),
            #[cfg(feature = "egl")]
            PlatformContext::Egl(context) => context
                .with_surface(callback)
                .unwrap_or_else(|error| error!("Failed to draw on a surface: {}", error)),
            #[cfg(target_os = "emscripten")]
            PlatformContext::WebGL(context) => {
                context
                    .with_surface(callback)
                    .unwrap_or_else(|error| error!("Failed to draw on a surface: {}", error));
            }
            PlatformContext::Unsupported => {}
        }
    }

    pub fn resize_surface(&mut self, size: ISize) {
        match self {
            #[cfg(target_os = "macos")]
            PlatformContext::Metal(context) => context.resize_surface(size),
            #[cfg(feature = "d3d")]
            PlatformContext::D3D(context) => context.resize(size),
            #[cfg(feature = "angle")]
            PlatformContext::Angle(context) => context
                .resize_surface(size)
                .unwrap_or_else(|error| error!("Failed to resize a surface: {:?}", error)),
            #[cfg(feature = "x11")]
            PlatformContext::XlibGl(context) => context
                .resize_surface(size)
                .unwrap_or_else(|error| error!("Failed to resize a surface: {:?}", error)),
            #[cfg(feature = "egl")]
            PlatformContext::Egl(context) => context
                .resize_surface(size)
                .unwrap_or_else(|error| error!("Failed to resize a surface: {:?}", error)),
            #[cfg(target_os = "emscripten")]
            PlatformContext::WebGL(context) => context
                .resize_surface(size)
                .unwrap_or_else(|error| error!("Failed to resize a surface: {}", error)),
            PlatformContext::Unsupported => {}
        }
    }

    #[cfg(all(feature = "raw-window-handle-5", not(feature = "raw-window-handle-6")))]
    pub fn for_window_handle<
        W: raw_window_handle_5::HasRawDisplayHandle + raw_window_handle_5::HasRawWindowHandle,
    >(
        w: &W,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        unsafe { Self::for_window_handle_5(w, width, height) }
    }

    #[cfg(all(feature = "raw-window-handle-6", not(feature = "raw-window-handle-5")))]
    pub fn for_window_handle<
        W: raw_window_handle_6::HasDisplayHandle + raw_window_handle_6::HasWindowHandle,
    >(
        w: &W,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        unsafe { Self::for_window_handle_6(w, width, height) }
    }

    #[cfg(feature = "raw-window-handle-5")]
    pub unsafe fn for_window_handle_5<
        W: raw_window_handle_5::HasRawDisplayHandle + raw_window_handle_5::HasRawWindowHandle,
    >(
        w: &W,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        use raw_window_handle_5::RawWindowHandle;

        let window_handle = w.raw_window_handle();
        match &window_handle {
            #[cfg(target_os = "macos")]
            RawWindowHandle::AppKit(handle) => Ok(new_metal_context(handle.ns_view, width, height)),
            #[cfg(target_os = "emscripten")]
            RawWindowHandle::Web(handle) => new_webgl_context(handle.id, width, height),
            _ => bail!("Unsupported platform: {:?}", &window_handle),
        }
    }

    #[cfg(feature = "raw-window-handle-6")]
    pub unsafe fn for_window_handle_6<
        W: raw_window_handle_6::HasDisplayHandle + raw_window_handle_6::HasWindowHandle,
    >(
        w: &W,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        use raw_window_handle_6::RawWindowHandle;

        let window_handle = w
            .window_handle()
            .map_err(|error| anyhow!("Failed to get window_handle: {}", error))?;
        match window_handle.as_raw() {
            #[cfg(target_os = "macos")]
            RawWindowHandle::AppKit(handle) => {
                Ok(new_metal_context(handle.ns_view.as_ptr(), width, height))
            }
            #[cfg(target_os = "emscripten")]
            RawWindowHandle::Web(handle) => new_webgl_context(handle.id, width, height),
            _ => bail!("Unsupported platform: {:?}", &window_handle),
        }
    }
}

#[cfg(target_os = "macos")]
fn new_metal_context(ns_view: *mut std::ffi::c_void, width: u32, height: u32) -> PlatformContext {
    use cocoa::base::id as cocoa_id;
    use core_graphics_types::geometry::CGSize;

    PlatformContext::Metal(crate::MetalContext::new(
        ns_view as cocoa_id,
        Some(CGSize::new(width.into(), height.into())),
    ))
}

#[cfg(target_os = "emscripten")]
fn new_webgl_context(id: u32, width: u32, height: u32) -> Result<PlatformContext> {
    Ok(PlatformContext::WebGL(crate::WebGLContext::new(
        id,
        width as i32,
        height as i32,
    )?))
}
