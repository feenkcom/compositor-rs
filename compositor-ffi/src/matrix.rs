use array_box::ArrayBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

use compositor::{Matrix, Scalar};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_matrix_new(
    values: BorrowedPtr<ArrayBox<f32>>,
) -> OwnedPtr<Matrix> {
    values
        .with_ref_ok(|values| {
            let buffer: &[f32; 9] = values.to_slice().try_into().unwrap();
            let buffer = &unsafe { *(buffer as *const [f32; 9] as *const [Scalar; 9]) };

            OwnedPtr::new(Matrix::from_9(*buffer))
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_matrix_drop(matrix: OwnedPtr<Matrix>) {
    drop(matrix);
}
