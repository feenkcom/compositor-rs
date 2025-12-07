use std::sync::Arc;

use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

use compositor::Layer;

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_clone(
    layer: *mut ValueBox<Arc<dyn Layer>>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    layer
        .with_ref_ok(|layer| ValueBox::new(layer.clone_arc()))
        .into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_debug(layer: *mut ValueBox<Arc<dyn Layer>>) -> *mut ValueBox<StringBox> {
    layer
        .with_ref_ok(|layer| ValueBox::new(StringBox::from_string(format!("{:#?}", layer))))
        .into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_with_layers(
    layer: *mut ValueBox<Arc<dyn Layer>>,
    layers: *mut ValueBox<Vec<Arc<dyn Layer>>>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    layer
        .with_ref(|layer| {
            layers
                .take_value()
                .map(|layers| ValueBox::new(layer.with_layers(layers)))
        })
        .into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_count_layers(layer_ptr: *mut ValueBox<Arc<dyn Layer>>) -> usize {
    layer_ptr
        .with_ref_ok(|layer| layer.count_layers())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_count_refs(layer_ptr: *mut ValueBox<Arc<dyn Layer>>) -> usize {
    layer_ptr
        .with_ref_ok(|layer| Arc::strong_count(&layer) - 1)
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_drop(ptr: *mut ValueBox<Arc<dyn Layer>>) {
    ptr.release();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layers_new() -> *mut ValueBox<Vec<Arc<dyn Layer>>> {
    ValueBox::new(vec![]).into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layers_add(
    layers: *mut ValueBox<Vec<Arc<dyn Layer>>>,
    layer: *mut ValueBox<Arc<dyn Layer>>,
) {
    layers
        .with_mut(|layers| {
            layer.with_clone_ok(|layer| {
                layers.push(layer);
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layers_drop(ptr: *mut ValueBox<Vec<Arc<dyn Layer>>>) {
    ptr.release();
}
