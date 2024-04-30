use std::error::Error;
use std::ffi::{c_int, c_void, CString};

use anyhow::Result;
use skia_safe::gpu::gl::{Enum, FramebufferInfo, Interface, UInt};
use skia_safe::gpu::Protected;
use skia_safe::gpu::{BackendRenderTarget, ContextOptions, DirectContext, SurfaceOrigin};
use skia_safe::{gpu, ColorType, ISize, Surface};

use crate::webgl_utils::*;
use crate::{PlatformCompositor, PlatformContext};

type GLenum = i32;
type GLint = i32;
type GLuint = u32;
type GLsizei = u32;

// See https://chromium.googlesource.com/external/skia/gpu/+/refs/heads/master/include/GrGLDefines.h
const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
const GL_RGBA8: GLenum = 0x8058;
const GL_TRUE: c_int = 1;
const GL_FALSE: c_int = 0;

// See https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGet.xhtml
type GlGetIntegerv = unsafe extern "C" fn(pname: GLenum, data: *mut GLint);
// https://registry.khronos.org/OpenGL-Refpages/es2.0/xhtml/glClearStencil.xml
type GlClearStencil = unsafe extern "C" fn(s: GLint);
type GlViewport = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub struct WebGLContext {
    target_id: u32,
    gl: Gl,
    webgl_context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
    egl_context: Option<WebGLWindowContext>,
    width: i32,
    height: i32,
}

impl WebGLContext {
    pub fn new(id: u32, width: i32, height: i32) -> Result<Self> {
        // Context configurations
        let mut attrs = EmscriptenWebGLContextAttributes::default();
        unsafe { emscripten_webgl_init_context_attributes(&mut attrs) };
        attrs.alpha = em_bool(true);
        attrs.premultipliedAlpha = em_bool(true);
        attrs.majorVersion = 2;
        attrs.enableExtensionsByDefault = em_bool(true);

        info!("Initialized WebGL attributes: {:?}", &attrs);

        let target_id = CString::new("#canvas").unwrap();
        debug!(
            "About to create WebGL context for target: {}({})",
            id,
            target_id.to_string_lossy()
        );
        let context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE =
            unsafe { emscripten_webgl_create_context(target_id.as_ptr(), &attrs) };
        if (context < 0) {
            bail!("Failed to create webgl context for target: {}", id);
        }

        let r: EMSCRIPTEN_RESULT = unsafe { emscripten_webgl_make_context_current(context) };
        if (r < 0) {
            bail!("Failed to make webgl current");
        }

        let gl = Gl::new()?;

        let mut context = Self {
            target_id: id,
            gl,
            webgl_context: context,
            egl_context: None,
            width,
            height,
        };

        context.initialize_context()?;
        Ok(context)
    }

    pub fn with_surface(
        &mut self,
        callback: impl FnOnce(&mut Surface),
    ) -> Result<(), Box<dyn Error>> {
        self.make_current()?;

        if let Some(surface) = self.get_surface() {
            callback(surface);
            self.flush_and_submit();
        }
        self.swap_buffers()?;
        self.make_not_current()?;

        Ok(())
    }

    pub fn resize_surface(&mut self, size: ISize) -> Result<()> {
        self.width = size.width;
        self.height = size.height;

        self.make_current()?;
        self.destroy_context()?;
        self.initialize_context()?;
        Ok(())
    }

    fn get_surface(&mut self) -> Option<&mut Surface> {
        if let Some(ref mut egl_context) = self.egl_context {
            if egl_context.surface.is_none() {
                match egl_context.try_create_surface(&self.gl, (self.width, self.height)) {
                    Ok(_) => {}
                    Err(error) => {
                        error!("Failed to initialize surface: {:?}", error);
                    }
                };
            }
            return egl_context.surface.as_mut();
        }
        None
    }

    fn initialize_context(&mut self) -> Result<()> {
        if self.egl_context.is_some() {
            bail!("Context already initialized");
        }
        self.egl_context = Some(WebGLWindowContext::try_create(self.webgl_context)?);
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

    fn flush_and_submit(&mut self) {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.direct_context.flush_and_submit();
        }
    }

    fn swap_buffers(&mut self) -> Result<()> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.swap_buffers()?;
        }
        Ok(())
    }
}

impl Drop for WebGLContext {
    fn drop(&mut self) {
        match self.destroy_context() {
            Ok(_) => {}
            Err(error) => {
                error!("Failed to destroy context: {}", error)
            }
        }
    }
}

#[derive(Debug)]
struct Gl {
    gl_get_integerv: GlGetIntegerv,
    gl_clear_stencil: GlClearStencil,
    gl_viewport: GlViewport,
}

impl Gl {
    pub fn new() -> Result<Self> {
        Ok(Self {
            gl_get_integerv: get_proc_address("glGetIntegerv")
                .map(|addr| unsafe { std::mem::transmute(addr) })
                .ok_or_else(|| anyhow!("Could not find glGetIntegerv"))?,
            gl_clear_stencil: get_proc_address("glClearStencil")
                .map(|addr| unsafe { std::mem::transmute(addr) })
                .ok_or_else(|| anyhow!("Could not find glClearStencil"))?,
            gl_viewport: get_proc_address("glViewport")
                .map(|addr| unsafe { std::mem::transmute(addr) })
                .ok_or_else(|| anyhow!("Could not find glViewport"))?,
        })
    }
}

struct WebGLWindowContext {
    webgl_context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
    backend_context: Interface,
    direct_context: DirectContext,
    surface: Option<Surface>,
}

impl WebGLWindowContext {
    fn try_create(webgl_context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE) -> Result<Self> {
        let r = unsafe { emscripten_webgl_make_context_current(webgl_context) };
        if r < 0 {
            bail!("Failed to make WebGL context current");
        }

        let interface =
            Interface::new_native().ok_or_else(|| anyhow!("Failed to create native Interface"))?;

        let context_options = ContextOptions::default();
        let direct_context = DirectContext::new_gl(interface.clone(), &context_options)
            .ok_or_else(|| anyhow!("Failed to create direct context"))?;

        Ok(Self {
            webgl_context,
            backend_context: interface,
            direct_context,
            surface: None,
        })
    }

    fn try_create_surface(&mut self, gl: &Gl, size: (i32, i32)) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut surface) = self.surface {
            return Ok(());
        }

        let mut buffer: GLint = 0;
        unsafe { (gl.gl_get_integerv)(GL_FRAMEBUFFER_BINDING, &mut buffer) };

        let framebuffer_info = FramebufferInfo {
            fboid: buffer as UInt,
            format: GL_RGBA8 as Enum,
            protected: Protected::No,
        };

        let backend_render_target =
            gpu::backend_render_targets::make_gl(size, 0, 8, framebuffer_info);

        let surface = gpu::surfaces::wrap_backend_render_target(
            &mut self.direct_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        );

        self.surface = surface;
        self.surface
            .as_ref()
            .ok_or_else(|| anyhow!("Failed to create skia Surface"))?;
        Ok(())
    }

    fn destroy_context(&mut self) -> Result<()> {
        //self.make_not_current()?;
        Ok(())
    }

    fn make_current(&self) -> Result<()> {
        let r = unsafe { emscripten_webgl_make_context_current(self.webgl_context) };
        if r < 0 {
            bail!("Failed to make WebGL context current")
        }
        Ok(())
    }

    fn make_not_current(&self) -> Result<()> {
        let r = unsafe { emscripten_webgl_make_context_current(0) };
        if r < 0 {
            bail!("Failed to make WebGL context not current")
        }
        Ok(())
    }

    fn swap_buffers(&self) -> Result<()> {
        // egl.swap_buffers(self.egl_display, self.egl_surface)?;
        Ok(())
    }
}
