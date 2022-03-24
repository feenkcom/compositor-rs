use boxer::{ValueBox, ValueBoxPointer};
use compositor::{ClipLayer, Geometry, Layer, Point};
use std::sync::Arc;

#[no_mangle]
pub fn compositor_clip_layer_none() -> *mut ValueBox<Arc<dyn Layer>> {
    ValueBox::new(Arc::new(ClipLayer::none()) as Arc<dyn Layer>).into_raw()
}

#[no_mangle]
pub fn compositor_clip_layer_new(
    geometry: *mut ValueBox<Geometry>,
    offset_x: f32,
    offset_y: f32,
) -> *mut ValueBox<Arc<dyn Layer>> {
    geometry.with_not_null_value_return(std::ptr::null_mut(), |geometry| {
        let layer = ClipLayer::new(geometry, Point::new_f32(offset_x, offset_y));

        ValueBox::new(Arc::new(layer) as Arc<dyn Layer>).into_raw()
    })
}
