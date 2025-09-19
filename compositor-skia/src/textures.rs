use compositor_skia_platform::Platform;
use compositor_texture::TextureDesc;
use skia_safe::gpu::{BackendAPI, BackendTexture, RecordingContext};
use skia_safe::Size;

pub fn disassemble_backend_texture(
    platform: Option<&Platform>,
    context: &mut RecordingContext, // optional but handy for flushing
    backend_texture: &BackendTexture,
    scale: Size,
) -> Option<TextureDesc> {
    let platform = platform?;

    match backend_texture.backend() {
        BackendAPI::OpenGL => None,
        BackendAPI::Vulkan => None,
        #[cfg(feature = "metal")]
        BackendAPI::Metal => metal::disassemble_metal_backend_texture(
            platform.try_as_metal_platform()?,
            context,
            backend_texture,
            scale
        )
        .map(|texture| texture.into_texture()),
        BackendAPI::Direct3D => None,
        _ => None,
    }
}

/// Extracts Metal internals from a Skia BackendTexture for FFI.
/// Safety/lifetime: the returned pointer is borrowed.
/// If the receiver needs to keep it, they must retain it on the ObjC side.
#[cfg(feature = "metal")]
mod metal {
    use compositor_skia_platform::MetalPlatform;
    use compositor_texture::MetalTextureDesc;
    use foreign_types_shared::ForeignTypeRef;
    use skia_safe::gpu::{
        backend_formats, backend_textures, mtl, BackendAPI, BackendTexture, DirectContext,
        RecordingContext,
    };
    use std::ffi::c_void;
    use skia_safe::Size;

    pub(crate) fn disassemble_metal_backend_texture(
        platform: &MetalPlatform,
        _ctx: &mut RecordingContext, // optional but handy for flushing
        backend_tex: &BackendTexture,
        scale: Size,
    ) -> Option<MetalTextureDesc> {
        // Get the Metal-specific texture info (snapshot)
        let mtl_info: mtl::TextureInfo = backend_textures::get_mtl_texture_info(backend_tex)?;

        // Raw id<MTLTexture> as *const c_void
        let tex_ptr: mtl::Handle = mtl_info.texture();

        // Convert Skia’s BackendFormat to MTLPixelFormat (u32)
        let mtl_fmt: mtl::PixelFormat =
            backend_formats::as_mtl_format(&backend_tex.backend_format())?;

        let texture = Some(MetalTextureDesc {
            device: platform.device.as_ptr() as _,
            queue: platform.queue.as_ptr() as _,
            texture: tex_ptr as *const c_void,
            width: backend_tex.width(),
            height: backend_tex.height(),
            scale_width: scale.width,
            scale_height: scale.height,
            mipmapped: backend_tex.has_mipmaps(),
            pixel_format: mtl_fmt,
        });

        texture
    }
}
