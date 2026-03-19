use std::sync::Arc;

use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

use compositor::Layer;

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_clone(
    layer: BorrowedPtr<Arc<dyn Layer>>,
) -> OwnedPtr<Arc<dyn Layer>> {
    layer
        .with_ref_ok(|layer| OwnedPtr::new(layer.clone_arc()))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_debug(
    layer: BorrowedPtr<Arc<dyn Layer>>,
) -> OwnedPtr<StringBox> {
    layer
        .with_ref_ok(|layer| OwnedPtr::new(StringBox::from_string(format!("{:#?}", layer))))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_with_layers(
    layer: BorrowedPtr<Arc<dyn Layer>>,
    layers: OwnedPtr<Vec<Arc<dyn Layer>>>,
) -> OwnedPtr<Arc<dyn Layer>> {
    layer
        .with_ref(|layer| {
            layers.with_value_ok(|layers| OwnedPtr::new(layer.with_layers(layers)))
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_count_layers(layer_ptr: BorrowedPtr<Arc<dyn Layer>>) -> usize {
    layer_ptr
        .with_ref_ok(|layer| layer.count_layers())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_count_refs(layer_ptr: BorrowedPtr<Arc<dyn Layer>>) -> usize {
    layer_ptr
        .with_ref_ok(|layer| Arc::strong_count(&layer) - 1)
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layer_drop(ptr: OwnedPtr<Arc<dyn Layer>>) {
    drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layers_new() -> OwnedPtr<Vec<Arc<dyn Layer>>> {
    OwnedPtr::new(vec![])
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_layers_add(
    mut layers: BorrowedPtr<Vec<Arc<dyn Layer>>>,
    layer: BorrowedPtr<Arc<dyn Layer>>,
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
pub extern "C" fn compositor_layers_drop(ptr: OwnedPtr<Vec<Arc<dyn Layer>>>) {
    drop(ptr);
}
