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
    match backend_texture.backend() {
        #[cfg(any(feature = "egl", feature = "angle"))]
        BackendAPI::OpenGL => {
            egl::disassemble_egl_backend_texture(platform?, context, backend_texture, scale)
                .map(|texture| texture.into_texture())
        }
        #[cfg(not(any(feature = "egl", feature = "angle")))]
        BackendAPI::OpenGL => None,
        BackendAPI::Vulkan => None,
        #[cfg(feature = "metal")]
        BackendAPI::Metal => metal::disassemble_metal_backend_texture(
            platform?.try_as_metal_platform()?,
            context,
            backend_texture,
            scale,
        )
        .map(|texture| texture.into_texture()),
        #[cfg(not(feature = "metal"))]
        BackendAPI::Metal => None,
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
        backend_formats, backend_textures, mtl, BackendTexture, RecordingContext,
    };
    use skia_safe::Size;
    use std::ffi::c_void;

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

        // Convert Skia's BackendFormat to MTLPixelFormat (u32)
        let mtl_fmt: mtl::PixelFormat =
            backend_formats::as_mtl_format(&backend_tex.backend_format())?;

        Some(MetalTextureDesc {
            device: platform.device.as_ptr() as _,
            queue: platform.queue.as_ptr() as _,
            texture: tex_ptr as *const c_void,
            width: backend_tex.width(),
            height: backend_tex.height(),
            scale_width: scale.width,
            scale_height: scale.height,
            mipmapped: backend_tex.has_mipmaps(),
            pixel_format: mtl_fmt,
        })
    }
}

/// Extracts OpenGL texture internals from a Skia BackendTexture for FFI.
#[cfg(any(feature = "egl", feature = "angle"))]
mod egl {
    use compositor_skia_platform::Platform;
    use compositor_texture::EglTextureDesc;
    use skia_safe::gpu::{backend_textures, BackendTexture, RecordingContext};
    use skia_safe::Size;

    pub(crate) fn disassemble_egl_backend_texture(
        platform: &Platform,
        _ctx: &mut RecordingContext, // optional but handy for flushing
        backend_tex: &BackendTexture,
        scale: Size,
    ) -> Option<EglTextureDesc> {
        let gl_info = backend_textures::get_gl_texture_info(backend_tex)?;
        let (display, context) = platform.try_as_egl_handles()?;

        Some(EglTextureDesc {
            display,
            context,
            texture_id: gl_info.id as u32,
            texture_target: gl_info.target as u32,
            texture_format: gl_info.format as u32,
            width: backend_tex.width(),
            height: backend_tex.height(),
            scale_width: scale.width,
            scale_height: scale.height,
            mipmapped: backend_tex.has_mipmaps(),
            is_protected: gl_info.is_protected(),
        })
    }
}
