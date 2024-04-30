use compositor::{ExplicitLayer, Layer};
use compositor_platform::{PlatformCompositor, PlatformContext};
use compositor_skia::SkiaDrawable;
use skia_safe::Color4f;
use winit_28::dpi::LogicalSize;
use winit_28::event::{Event, WindowEvent};
use winit_28::event_loop::{ControlFlow, EventLoop};
use winit_28::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Skia compositor")
        .with_inner_size(LogicalSize::new(800.0, 600.0))
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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        compositor.draw().unwrap();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
