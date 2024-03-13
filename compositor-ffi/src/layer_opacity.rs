use compositor::{Layer, OpacityLayer};
use std::sync::Arc;
use value_box::ValueBox;

#[no_mangle]
pub fn compositor_opacity_layer_new() -> *mut ValueBox<Arc<dyn Layer>> {
    ValueBox::new(Arc::new(OpacityLayer::new()) as Arc<dyn Layer>).into_raw()
}

#[no_mangle]
pub fn compositor_opacity_layer_new_alpha(alpha: f32) -> *mut ValueBox<Arc<dyn Layer>> {
    ValueBox::new(Arc::new(OpacityLayer::new_alpha(alpha)) as Arc<dyn Layer>).into_raw()
}
