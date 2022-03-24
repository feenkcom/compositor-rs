use boxer::{ValueBox, ValueBoxPointer};
use compositor::{Layer, Matrix, TransformationLayer};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[no_mangle]
pub fn compositor_transformation_layer_new() -> *mut ValueBox<Arc<dyn Layer>> {
    let layer: Arc<dyn Layer> = Rc::new(RefCell::new(TransformationLayer::new(
        Matrix::new_identity(),
    )));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn compositor_transformation_layer_new_matrix(
    matrix_ptr: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    matrix_ptr.with_not_null_value_return(std::ptr::null_mut(), |matrix| {
        let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(TransformationLayer::new(matrix)));
        ValueBox::new(layer).into_raw()
    })
}
