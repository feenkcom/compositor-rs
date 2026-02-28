use std::ffi::c_void;

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

    pub fn try_as_egl(&self) -> Option<&EglTextureDesc> {
        if self.backend != Backend::Egl {
            return None;
        }
        Some(unsafe { &*(self.texture as *const EglTextureDesc) })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Backend {
    Unsupported,
    Metal,
    Egl,
}

#[derive(Debug)]
#[repr(C)]
pub struct MetalTextureDesc {
    pub device: *const c_void,
    pub queue: *mut c_void,
    /// Raw id<MTLTexture> (borrowed).
    pub texture: *const c_void,
    pub width: i32,
    pub height: i32,
    pub scale_width: f32,
    pub scale_height: f32,
    pub mipmapped: bool,
    /// MTLPixelFormat as u32
    pub pixel_format: u32,
}

impl MetalTextureDesc {
    pub fn into_texture(self) -> TextureDesc {
        TextureDesc {
            backend: Backend::Metal,
            texture: Box::into_raw(Box::new(self)) as *const c_void,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct EglTextureDesc {
    /// Raw EGLDisplay.
    pub display: *mut c_void,
    /// Raw EGLContext owning the texture.
    pub context: *mut c_void,
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
    pub mipmapped: bool,
    /// Whether the texture is protected content.
    pub is_protected: bool,
}

impl EglTextureDesc {
    pub fn into_texture(self) -> TextureDesc {
        TextureDesc {
            backend: Backend::Egl,
            texture: Box::into_raw(Box::new(self)) as *const c_void,
        }
    }
}
