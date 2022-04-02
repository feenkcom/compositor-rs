mod driver;

use compositor::{
    Geometry, Layer, Point, Radius, Rectangle, Shadow, ShadowLayer, TransformationLayer,
};
use compositor_skia::{into_compositor_matrix, to_compositor_color, Cache, SkiaCompositor};
use skia_safe::Matrix;
use std::sync::Arc;

fn main() {
    env_logger::init();

    let shadow = Shadow::new(
        to_compositor_color(skia_safe::Color::BLACK),
        Radius::new(10.0, 10.0),
        Point::new_f32(200.0, 200.0),
        Geometry::Rectangle(Rectangle::extent(300.0, 200.0)),
    );

    let transformation_layer =
        TransformationLayer::new(into_compositor_matrix(&Matrix::rotate_deg(10.0)))
            .with_layer(Arc::new(ShadowLayer::new(shadow)));

    let mut cache = Cache::new();

    driver::run(move |canvas| {
        canvas.clear(skia_safe::Color::WHITE);

        let mut compositor = SkiaCompositor::new(canvas, &mut cache);

        transformation_layer.compose(&mut compositor);
    });
}
