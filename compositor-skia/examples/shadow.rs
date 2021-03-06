mod driver;

use compositor::{Geometry, Layer, Point, Radius, Rectangle, Shadow, ShadowLayer};
use compositor_skia::{to_compositor_color, Cache, SkiaCompositor};

fn main() {
    env_logger::init();

    let shadow = Shadow::new(
        to_compositor_color(skia_safe::Color::BLACK),
        Radius::new(10.0, 10.0),
        Point::new_f32(200.0, 200.0),
        Geometry::Rectangle(Rectangle::extent(300.0, 200.0)),
    );

    let shadow_layer = ShadowLayer::new(shadow);

    let mut cache = Cache::new();

    driver::run(move |canvas| {
        canvas.clear(skia_safe::Color::WHITE);

        let mut compositor = SkiaCompositor::new(canvas, &mut cache);

        shadow_layer.compose(&mut compositor);
    });
}
