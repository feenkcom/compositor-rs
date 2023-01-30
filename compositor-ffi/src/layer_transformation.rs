use compositor::{Layer, Matrix, TransformationLayer};
use std::sync::Arc;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn compositor_transformation_layer_new(
    matrix: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    matrix
        .take_value()
        .map(|matrix| Arc::new(TransformationLayer::new(matrix)) as Arc<dyn Layer>)
        .into_raw()
}
