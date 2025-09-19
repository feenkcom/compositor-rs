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
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Backend {
    Unsupported,
    Metal,
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
