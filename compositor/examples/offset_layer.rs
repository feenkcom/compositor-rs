#![feature(min_specialization)]

use compositor::{OffsetLayer, Point};
use phlow::{import_extensions, phlow};

use compositor::CompositorExtensions;
import_extensions!(CompositorExtensions);

fn main() {
    let layer = OffsetLayer::new_offset(Point::new(20.0, 30.0));
    phlow_server::serve(phlow!(layer)).join().unwrap();
}
