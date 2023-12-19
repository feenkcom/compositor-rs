#![feature(min_specialization)]

use compositor::{Extent, Point, TiledLayer, TiledLayerFigure};
use phlow::{import_extensions, phlow};

use compositor::CompositorExtensions;
import_extensions!(CompositorExtensions);

fn main() {
    let layer = TiledLayer::default();
    layer.add_figure(TiledLayerFigure::new(
        1,
        Point::new(10.0, 5.0),
        Extent::new(100.0, 50.0),
    ));
    layer.add_figure(TiledLayerFigure::new(
        2,
        Point::new(50.0, 20.0),
        Extent::new(100.0, 50.0),
    ));
    layer.add_figure(TiledLayerFigure::new(
        3,
        Point::new(30.0, 0.0),
        Extent::new(100.0, 50.0),
    ));
    layer.add_figure(TiledLayerFigure::new(
        4,
        Point::new(100.0, 80.0),
        Extent::new(100.0, 50.0),
    ));
    layer.add_figure(TiledLayerFigure::new(
        5,
        Point::new(900.0, 800.0),
        Extent::new(100.0, 50.0),
    ));
    phlow_server::serve(phlow!(layer)).join().unwrap();
}
