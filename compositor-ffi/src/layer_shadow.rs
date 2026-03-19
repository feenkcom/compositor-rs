use std::sync::Arc;

use value_box::{OwnedPtr, ReturnBoxerResult};

use compositor::{Layer, Shadow, ShadowLayer};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_shadow_layer_new(
    shadow: OwnedPtr<Shadow>,
) -> OwnedPtr<Arc<dyn Layer>> {
    shadow
        .with_value_ok(|shadow| OwnedPtr::new(Arc::new(ShadowLayer::new(shadow)) as Arc<dyn Layer>))
        .or_log(OwnedPtr::null())
}
