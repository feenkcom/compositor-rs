#![feature(min_specialization)]

use compositor::{Extent, Picture, PictureLayer, Point, TiledLayer, TiledLayerFigure};
use phlow::{import_extensions, phlow};
use rand::Rng;
use skia_safe::{Color4f, Paint, PictureRecorder, Rect};
use std::sync::Arc;

use compositor::CompositorExtensions;
use compositor_skia::{CompositorSkiaExtensions, SkiaPicture};
import_extensions!(CompositorExtensions, CompositorSkiaExtensions);

fn main() {
    let mut rng = rand::thread_rng();

    let mut pictures =
        (0..5)
            .into_iter()
            .map(|i| {
                let mut recorder = PictureRecorder::new();
                let canvas = recorder.begin_recording(&Rect::new(0.0, 0.0, 100.0, 50.0), None);
                let paint = Paint::new(&Color4f::new(rng.gen(), rng.gen(), rng.gen(), 1.0), None);
                canvas.draw_rect(&Rect::new(0.0, 0.0, 100.0, 50.0), &paint);
                let picture = recorder.finish_recording_as_picture(None).unwrap();
                PictureLayer::new(Arc::new(SkiaPicture::new(picture)), false)
            })
            .collect::<Vec<PictureLayer>>();

    let layer = TiledLayer::default();
    layer.add_figure(
        TiledLayerFigure::new(1, Point::new(10.0, 5.0), Extent::new(100.0, 50.0))
            .with_picture(pictures.remove(0)),
    );
    layer.add_figure(
        TiledLayerFigure::new(2, Point::new(150.0, 220.0), Extent::new(100.0, 50.0))
            .with_picture(pictures.remove(0)),
    );
    layer.add_figure(
        TiledLayerFigure::new(3, Point::new(-180.0, -120.0), Extent::new(100.0, 50.0))
            .with_picture(pictures.remove(0)),
    );
    layer.add_figure(
        TiledLayerFigure::new(4, Point::new(100.0, 80.0), Extent::new(100.0, 50.0))
            .with_picture(pictures.remove(0)),
    );
    layer.add_figure(
        TiledLayerFigure::new(5, Point::new(900.0, 800.0), Extent::new(100.0, 50.0))
            .with_picture(pictures.remove(0)),
    );
    phlow_server::serve(phlow!(layer)).join().unwrap();
}
