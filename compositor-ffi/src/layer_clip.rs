use std::sync::Arc;

use value_box::{ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

use compositor::{ClipLayer, Geometry, Layer, Point};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_clip_layer_none() -> *mut ValueBox<Arc<dyn Layer>> {
    ValueBox::new(Arc::new(ClipLayer::none()) as Arc<dyn Layer>).into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_clip_layer_new(
    geometry: *mut ValueBox<Geometry>,
    offset_x: f32,
    offset_y: f32,
) -> *mut ValueBox<Arc<dyn Layer>> {
    geometry
        .with_clone_ok(|geometry| {
            let layer = ClipLayer::new(geometry, Point::new_f32(offset_x, offset_y));
            ValueBox::new(Arc::new(layer) as Arc<dyn Layer>)
        })
        .into_raw()
}
