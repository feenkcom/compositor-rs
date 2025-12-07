use compositor::{Layer, Matrix, TransformationLayer};
use std::sync::Arc;
use value_box::{ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_transformation_layer_new(
    matrix: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    matrix
        .take_value()
        .map(|matrix| ValueBox::new(Arc::new(TransformationLayer::new(matrix)) as Arc<dyn Layer>))
        .into_raw()
}
