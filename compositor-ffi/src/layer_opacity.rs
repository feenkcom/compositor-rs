use compositor::{Layer, OpacityLayer};
use std::sync::Arc;
use value_box::OwnedPtr;

#[unsafe(no_mangle)]
pub extern "C" fn compositor_opacity_layer_new() -> OwnedPtr<Arc<dyn Layer>> {
    OwnedPtr::new(Arc::new(OpacityLayer::new()) as Arc<dyn Layer>)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_opacity_layer_new_alpha(alpha: f32) -> OwnedPtr<Arc<dyn Layer>> {
    OwnedPtr::new(Arc::new(OpacityLayer::new_alpha(alpha)) as Arc<dyn Layer>)
}
