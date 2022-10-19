use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer};
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
pub fn compositor_layer_debug(layer: *mut ValueBox<Arc<dyn Layer>>) -> *mut ValueBox<BoxerString> {
    layer.with_not_null_return(std::ptr::null_mut(), |layer| {
        ValueBox::new(BoxerString::from_string(format!("{:#?}", layer))).into_raw()
    })
}

#[no_mangle]
pub fn compositor_layer_with_layers(
    layer: *mut ValueBox<Arc<dyn Layer>>,
    mut layers: *mut ValueBox<Vec<Arc<dyn Layer>>>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    layer.with_not_null_return(std::ptr::null_mut(), |layer| {
        layers.with_not_null_value_consumed_return(std::ptr::null_mut(), |layers| {
            ValueBox::new(layer.with_layers(layers)).into_raw()
        })
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
pub fn compositor_layer_drop(ptr: *mut ValueBox<Arc<dyn Layer>>) {
    ptr.release();
}

#[no_mangle]
pub fn compositor_layers_new() -> *mut ValueBox<Vec<Arc<dyn Layer>>> {
    ValueBox::new(vec![]).into_raw()
}

#[no_mangle]
pub fn compositor_layers_add(
    layers: *mut ValueBox<Vec<Arc<dyn Layer>>>,
    layer: *mut ValueBox<Arc<dyn Layer>>,
) {
    layers.with_not_null(|layers| {
        layer.with_not_null_value(|layer| {
            layers.push(layer);
        })
    })
}

#[no_mangle]
pub fn compositor_layers_drop(ptr: *mut ValueBox<Vec<Arc<dyn Layer>>>) {
    ptr.release();
}
