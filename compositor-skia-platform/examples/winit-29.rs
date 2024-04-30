use skia_safe::Color4f;
use winit_29::dpi::LogicalSize;
use winit_29::event::{Event, WindowEvent};
use winit_29::event_loop::EventLoop;
use winit_29::window::WindowBuilder;

use compositor::{ExplicitLayer, Layer};
use compositor_skia::SkiaDrawable;
use compositor_skia_platform::{PlatformCompositor, PlatformContext};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Skia compositor")
        .with_inner_size(LogicalSize::new(600.0, 400.0))
        .build(&event_loop)
        .unwrap();

    let context: PlatformContext = PlatformContext::for_window_handle(
        &window,
        window.inner_size().width,
        window.inner_size().height,
    )
    .unwrap();
    let mut compositor = PlatformCompositor::new(context);

    compositor
        .submit_layer(
            ExplicitLayer::new(SkiaDrawable::dynamic(|canvas| {
                canvas.draw_color(Color4f::new(0.0, 0.0, 1.0, 1.0), None);
            }))
            .clone_arc(),
        )
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
