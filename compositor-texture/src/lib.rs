mod types;

use std::ffi::c_void;
use std::os::raw;

pub use types::*;

#[derive(Debug)]
#[repr(C)]
pub struct TextureDesc {
    pub backend: Backend,
    pub texture: *const c_void,
}

impl TextureDesc {
    pub fn try_as_metal(&self) -> Option<&MetalTextureDesc> {
        if self.backend != Backend::Metal {
            return None;
        }
        Some(unsafe { &*(self.texture as *const MetalTextureDesc) })
    }

    pub fn try_as_opengl(&self) -> Option<&OpenGlDesc> {
        if self.backend != Backend::OpenGL {
            return None;
        }
        Some(unsafe { &*(self.texture as *const OpenGlDesc) })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Backend {
    Unsupported,
    Metal,
    OpenGL,
}

#[derive(Debug)]
#[repr(C)]
pub struct MetalTextureDesc {
    pub device: *const c_void,
    pub queue: *mut c_void,
    /// Raw id<MTLTexture> (borrowed).
    pub texture: *const c_void,
    /// MTLPixelFormat as u32
    pub pixel_format: u32,

    pub width: i32,
    pub height: i32,
    pub scale_width: f32,
    pub scale_height: f32,
    pub color_type: ColorType,
    pub mipmapped: bool,
    /// Whether the texture is protected content.
    pub protected: Protected,
}

impl MetalTextureDesc {
    pub fn into_texture(self) -> TextureDesc {
        TextureDesc {
            backend: Backend::Metal,
            texture: Box::into_raw(Box::new(self)) as *const c_void,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct OpenGlDesc {
    /// Raw GL_Display.
    pub display: *mut c_void,
    /// Raw GL_Context owning the texture.
    pub context: *mut c_void,
    pub surface: *mut c_void,
    pub get_proc_address: unsafe extern "C" fn(name: *const raw::c_char) -> *const c_void,
    pub get_current_context: unsafe extern "C" fn() -> *const c_void,
    pub texture_desc: OpenGlTextureDesc,
    pub framebuffer_desc: OpenGlFramebufferDesc,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct OpenGlTextureDesc {
    /// Raw GLuint texture id.
    pub texture_id: u32,
    /// GLenum texture target (for example GL_TEXTURE_2D).
    pub texture_target: u32,
    /// GLenum texture internal format (for example GL_RGBA8).
    pub texture_format: u32,
    pub width: i32,
    pub height: i32,
    pub scale_width: f32,
    pub scale_height: f32,
    pub color_type: ColorType,
    pub mipmapped: bool,
    /// Whether the texture is protected content.
    pub protected: Protected,
}

impl OpenGlDesc {
    pub fn into_texture(self) -> TextureDesc {
        TextureDesc {
            backend: Backend::OpenGL,
            texture: Box::into_raw(Box::new(self)) as *const c_void,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct OpenGlFramebufferDesc {
    pub fbo_id: u32,
    pub width: i32,
    pub height: i32,

    /// GL internal format for the FBO color attachment (e.g. GL_RGBA8 / GL_SRGB8_ALPHA8)
    /// This is the value Skia reports in FramebufferInfo.format.
    pub format: u32,

    pub sample_count: usize, // 1 unless MSAA
    pub stencil_bits: usize, // 0 or 8 typically
    pub protected: Protected,
}
