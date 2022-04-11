mod driver;

use compositor::{Layer, PictureLayer, TransformationLayer};
use compositor_skia::{into_compositor_matrix, Cache, SkiaCompositor, SkiaPicture};
use skia_safe::{Color, Matrix, Paint, Picture, PictureRecorder, Rect, Size};
use std::sync::Arc;

fn record_picture() -> Picture {
    let mut recorder = PictureRecorder::new();
    let canvas = recorder.begin_recording(Rect::from_size(Size::new(300.0, 200.0)), None);

    let mut paint = Paint::default();
    paint.set_color(Color::BLUE);
    canvas.draw_rect(Rect::from_size(Size::new(300.0, 200.0)), &paint);
    recorder.finish_recording_as_picture(None).unwrap()
}

fn main() {
    env_logger::init();

    let picture_layer = PictureLayer::new(Arc::new(SkiaPicture::new(record_picture())), true);
    let transformation_layer =
        TransformationLayer::new(into_compositor_matrix(&Matrix::rotate_deg(10.0)))
            .with_layer(Arc::new(picture_layer));

    let mut cache = Cache::new();

    driver::run(move |canvas| {
        canvas.clear(skia_safe::Color::WHITE);

        let mut compositor = SkiaCompositor::new(canvas, &mut cache);

        transformation_layer.compose(&mut compositor);
    });
}
