use compositor::{Layer, OffsetLayer, Point};
use std::sync::Arc;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn compositor_offset_layer_new() -> *mut ValueBox<Arc<dyn Layer>> {
    ValueBox::new(Arc::new(OffsetLayer::new()) as Arc<dyn Layer>).into_raw()
}

#[no_mangle]
pub fn compositor_offset_layer_new_point(x: f32, y: f32) -> *mut ValueBox<Arc<dyn Layer>> {
    ValueBox::new(Arc::new(OffsetLayer::new_offset(Point::new_f32(x, y))) as Arc<dyn Layer>)
        .into_raw()
}

#[no_mangle]
pub fn compositor_offset_layer_with_point(
    layer: *mut ValueBox<Arc<dyn Layer>>,
    x: f32,
    y: f32,
) -> *mut ValueBox<Arc<dyn Layer>> {
    layer
        .with_ref_ok(|layer| {
            let offset_layer = layer
                .any()
                .downcast_ref::<OffsetLayer>()
                .expect("Is not an offset layer!");
            ValueBox::new(Arc::new(offset_layer.with_offset(Point::new_f32(x, y)))
                as Arc<dyn Layer>)
        })
        .into_raw()
}

#[no_mangle]
pub fn compositor_offset_layer_get_x(layer: *mut ValueBox<Arc<dyn Layer>>) -> f32 {
    layer
        .with_ref_ok(|layer| {
            let offset_layer = layer
                .any()
                .downcast_ref::<OffsetLayer>()
                .expect("Is not an offset layer!");

            offset_layer.offset().x().into()
        })
        .or_log(0.0)
}

#[no_mangle]
pub fn compositor_offset_layer_get_y(layer: *mut ValueBox<Arc<dyn Layer>>) -> f32 {
    layer
        .with_ref_ok(|layer| {
            let offset_layer = layer
                .any()
                .downcast_ref::<OffsetLayer>()
                .expect("Is not an offset layer!");

            offset_layer.offset().y().into()
        })
        .or_log(0.0)
}
