use cocoa::base::YES;
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::ForeignType;
use metal::Device;
use skia_safe::gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};
use skia_safe::{gpu, Color4f, ColorType};
use std::mem;
use winit_29::dpi::LogicalSize;
use winit_29::event::{Event, WindowEvent};
use winit_29::event_loop::EventLoop;
use winit_29::window::WindowBuilder;

use compositor::{ExplicitLayer, Layer, OffsetLayer, Texture, TextureLayer};
use compositor_skia::SkiaDrawable;
use compositor_skia_platform::{PlatformCompositor, PlatformContext, PlatformTexture};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Skia compositor")
        .with_inner_size(LogicalSize::new(600.0, 400.0))
        .build(&event_loop)
        .unwrap();

    let texture = PlatformTexture::offscreen(300, 200);
    if let Some(backend_texture) = texture.as_backend_texture() {
        let device = Device::system_default().expect("no device found");

        let command_queue = device.new_command_queue();

        let backend = unsafe {
            mtl::BackendContext::new(
                device.as_ptr() as mtl::Handle,
                command_queue.as_ptr() as mtl::Handle,
                std::ptr::null(),
            )
        };

        let mut direct_context = DirectContext::new_metal(&backend, None).unwrap();

        if let Some(mut surface) = gpu::surfaces::wrap_backend_texture(
            &mut direct_context,
            &backend_texture,
            SurfaceOrigin::TopLeft,
            None,
            ColorType::BGRA8888,
            None,
            None,
        ) {
            let canvas = surface.canvas();
            canvas.draw_color(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
        }
    }

    let context: PlatformContext = PlatformContext::for_window_handle(
        &window,
        window.inner_size().width,
        window.inner_size().height,
    )
    .unwrap();
    let mut compositor = PlatformCompositor::new(context);

    compositor
        .submit_layer(OffsetLayer::new().with_layers(
            vec![
                ExplicitLayer::new(SkiaDrawable::dynamic(|canvas| {
                    canvas.draw_color(Color4f::new(0.0, 0.0, 1.0, 1.0), None);
                })).clone_arc(),
                TextureLayer::new(texture.texture(), 300, 200).clone_arc()],
        ))
        .unwrap();

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, window_id } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            // Notify the windowing system that we'll be presenting to the window.
                            window.pre_present_notify();
                            compositor.draw().unwrap();
                        }
                        _ => (),
                    }
                }
                Event::AboutToWait => {
                    window.request_redraw();
                }

                _ => (),
            }
        })
        .unwrap();
}
