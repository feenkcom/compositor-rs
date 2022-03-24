use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use compositor::Layer;
use std::sync::Arc;

#[no_mangle]
pub fn compositor_layer_clone(
    layer_ptr: *mut ValueBox<Arc<dyn Layer>>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    layer_ptr.with_not_null_return(std::ptr::null_mut(), |layer| {
        ValueBox::new(layer.clone_arc()).into_raw()
    })
}

#[no_mangle]
pub fn compositor_layer_count_layers(layer_ptr: *mut ValueBox<Arc<dyn Layer>>) -> usize {
    layer_ptr.with_not_null_value_return(0, |layer| layer.count_layers())
}

#[no_mangle]
pub fn compositor_layer_count_refs(layer_ptr: *mut ValueBox<Arc<dyn Layer>>) -> usize {
    layer_ptr.with_not_null_value_return(0, |layer| Arc::strong_count(&layer) - 1)
}

#[no_mangle]
pub fn compositor_layer_drop(ptr: &mut *mut ValueBox<Arc<dyn Layer>>) {
    ptr.drop();
}
