use compositor::{Layer, Matrix, TransformationLayer};
use std::sync::Arc;
use value_box::{OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_transformation_layer_new(
    matrix: OwnedPtr<Matrix>,
) -> OwnedPtr<Arc<dyn Layer>> {
    matrix
        .with_value_ok(|matrix| OwnedPtr::new(Arc::new(TransformationLayer::new(matrix)) as Arc<dyn Layer>))
        .or_log(OwnedPtr::null())
}
