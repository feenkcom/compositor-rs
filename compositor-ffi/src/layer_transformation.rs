use boxer::{ValueBox, ValueBoxPointer};
use compositor::{Layer, Matrix, TransformationLayer};
use std::sync::Arc;

#[no_mangle]
pub fn compositor_transformation_layer_new(
    mut matrix: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    matrix.with_not_null_value_consumed_return(std::ptr::null_mut(), |matrix| {
        ValueBox::new(Arc::new(TransformationLayer::new(matrix)) as Arc<dyn Layer>).into_raw()
    })
}
