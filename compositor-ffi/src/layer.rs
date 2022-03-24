use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use compositor::{ClipLayer, Layer};
use std::cell::RefCell;
use std::rc::Rc;

#[no_mangle]
pub fn compositor_layer_add(
    _target_layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>,
    mut _child_layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>,
) {
    _target_layer_ptr.with_not_null_value(|target_layer| {
        _child_layer_ptr.with_not_null_value(|child_layer| {
            target_layer.borrow_mut().add_layer(child_layer);
        })
    })
}

#[no_mangle]
pub fn compositor_layer_count_layers(layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>) -> usize {
    layer_ptr.with_not_null_value_return(0, |layer| layer.borrow().count_layers())
}

#[no_mangle]
pub fn compositor_layer_count_refs(layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>) -> usize {
    layer_ptr.with_not_null_value_return(0, |layer| Rc::strong_count(&layer) - 1)
}

#[no_mangle]
pub fn compositor_layer_drop(ptr: &mut *mut ValueBox<Rc<RefCell<dyn Layer>>>) {
    ptr.drop();
}
