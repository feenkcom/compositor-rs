use compositor::Texture;

use skia_safe::gpu::{BackendTexture, Mipmapped};

pub struct PlatformTexture {
    texture: Texture,
    backend_texture: Option<BackendTexture>,
}

impl PlatformTexture {
    pub fn offscreen(width: u32, height: u32) -> Self {
        #[cfg(target_os = "macos")]
        {
            use cocoa::foundation::NSUInteger;
            use foreign_types_shared::ForeignType;
            use skia_safe::gpu::mtl;
            use metal::{Device, MTLPixelFormat, TextureDescriptor};

            let device = Device::system_default().expect("no device found");
            let texture_descriptor = TextureDescriptor::new();
            texture_descriptor.set_width(width as NSUInteger);
            texture_descriptor.set_height(height as NSUInteger);
            texture_descriptor.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

            let texture = device.new_texture(&texture_descriptor);

            let texture_info = unsafe { mtl::TextureInfo::new(texture.as_ptr() as mtl::Handle) };

            let backend_texture = unsafe {
                BackendTexture::new_metal(
                    (width as i32, height as i32),
                    Mipmapped::No,
                    &texture_info,
                )
            };

            return Self {
                texture: Texture::Metal(texture),
                backend_texture: Some(backend_texture),
            };
        }
        #[cfg(not(target_os = "macos"))]
        {
            return Self {
                texture: Texture::Unsupported,
                backend_texture: None,
            };
        }
    }

    pub fn texture(&self) -> Texture {
        self.texture.clone()
    }

    pub fn as_backend_texture(&self) -> Option<BackendTexture> {
        self.backend_texture.clone()
    }
}
