use std::sync::Arc;

use value_box::{ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

use compositor::{Layer, Shadow, ShadowLayer};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_shadow_layer_new(shadow: *mut ValueBox<Shadow>) -> *mut ValueBox<Arc<dyn Layer>> {
    shadow
        .take_value()
        .map(|shadow| ValueBox::new(Arc::new(ShadowLayer::new(shadow)) as Arc<dyn Layer>))
        .into_raw()
}
