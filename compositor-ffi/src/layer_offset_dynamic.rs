use compositor::{DynamicOffsetLayer, Layer, Point};
use std::ffi::c_void;
use std::sync::Arc;
use value_box::ValueBox;

#[unsafe(no_mangle)]
pub extern "C" fn compositor_dynamic_offset_layer_new(
    offset_fn: unsafe extern "C" fn(*mut c_void) -> Point,
    payload: *mut c_void,
    clone_fn: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    free_fn: unsafe extern "C" fn(*mut c_void),
) -> *mut ValueBox<Arc<dyn Layer>> {
    ValueBox::new(Arc::new(DynamicOffsetLayer::new(
        offset_fn, payload, clone_fn, free_fn,
    )) as Arc<dyn Layer>)
    .into_raw()
}
