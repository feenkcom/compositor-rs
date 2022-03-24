use boxer::{ValueBox, ValueBoxPointer};
use compositor::{Layer, Shadow, ShadowLayer};
use std::sync::Arc;

#[no_mangle]
pub fn compositor_shadow_layer_new(
    mut shadow: *mut ValueBox<Shadow>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    shadow.with_not_null_value_consumed_return(std::ptr::null_mut(), |shadow| {
        ValueBox::new(Arc::new(ShadowLayer::new(shadow)) as Arc<dyn Layer>).into_raw()
    })
}
