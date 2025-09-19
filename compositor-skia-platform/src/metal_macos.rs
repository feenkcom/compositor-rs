use std::mem;

use cocoa::appkit::NSView;
use cocoa::base::{id as cocoa_id, YES};
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal::{CommandQueue, Device, MTLPixelFormat, MetalDrawableRef, MetalLayer};
use skia_safe::gpu::mtl::BackendContext;
use skia_safe::gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};
use skia_safe::{gpu, scalar, ColorType, ISize, Size, Surface};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetalPlatform {
    pub device: Device,
    pub queue: CommandQueue,
}

#[derive(Debug)]
pub struct MetalContext {
    platform: MetalPlatform,
    layer: MetalLayer,
    backend_context: BackendContext,
    direct_context: DirectContext,
}

impl MetalContext {
    pub fn new(ns_view: cocoa_id, size: Option<CGSize>) -> Self {
        let device = Device::system_default().expect("no device found");

        let layer = {
            let layer = MetalLayer::new();
            layer.set_device(&device);
            layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
            layer.set_presents_with_transaction(false);
            if let Some(size) = size {
                layer.set_drawable_size(size);
            }

            unsafe {
                ns_view.setWantsLayer(YES);
                ns_view.setLayer(mem::transmute(layer.as_ref()));
            }
            layer
        };

        let queue = device.new_command_queue();

        let backend_context = unsafe {
            BackendContext::new(
                device.as_ptr() as mtl::Handle,
                queue.as_ptr() as mtl::Handle,
            )
        };

        let direct_context = DirectContext::new_metal(&backend_context, None).unwrap();
        //let direct_context = gpu::direct_contexts::make_metal(&backend_context, None).unwrap();

        MetalContext {
            platform: MetalPlatform { device, queue },
            layer,
            backend_context,
            direct_context,
        }
    }

    pub fn resize_surface(&mut self, size: ISize) {
        self.layer
            .set_drawable_size(CGSize::new(size.width.into(), size.height.into()));
    }

    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) {
        if let Some(drawable) = self.layer.next_drawable() {
            let drawable_size = {
                let size = self.layer.drawable_size();
                Size::new(size.width as scalar, size.height as scalar)
            };

            let texture_info =
                unsafe { mtl::TextureInfo::new(drawable.texture().as_ptr() as mtl::Handle) };

            let backend_render_target = BackendRenderTarget::new_metal(
                //let backend_render_target = gpu::backend_render_targets::make_mtl(
                (drawable_size.width as i32, drawable_size.height as i32),
                &texture_info,
            );

            if let Some(mut surface) = gpu::surfaces::wrap_backend_render_target(
                &mut self.direct_context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                ColorType::BGRA8888,
                None,
                None,
            ) {
                callback(&mut surface);

                self.direct_context.flush_and_submit();
                drop(surface);

                self.commit(drawable);
            };
        }
    }

    pub fn commit(&self, drawable: &MetalDrawableRef) {
        let command_buffer = self.platform.queue.new_command_buffer();
        command_buffer.present_drawable(drawable);
        command_buffer.commit()
    }
}
//
// pub struct MetalTexture {
//     texture: Texture,
//     backend_texture: BackendTexture,
// }
//
// impl MetalTexture {
//     pub fn offscreen(width: u32, height: u32) -> Self {
//         let device = Device::system_default().expect("no device found");
//         let texture_descriptor = TextureDescriptor::new();
//         texture_descriptor.set_width(width as NSUInteger);
//         texture_descriptor.set_height(height as NSUInteger);
//         texture_descriptor.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
//
//         let texture = device.new_texture(&texture_descriptor);
//
//         let texture_info = unsafe { mtl::TextureInfo::new(texture.as_ptr() as mtl::Handle) };
//
//         let backend_texture =
//             unsafe { BackendTexture::new_metal((width as i32, height as i32), Mipmapped::No, &texture_info) };
//
//         Self {
//             texture: Texture::Metal(texture),
//             backend_texture,
//         }
//     }
//
//     pub fn as_backend_texture(&self) -> BackendTexture {
//         self.backend_texture.clone()
//     }
// }
