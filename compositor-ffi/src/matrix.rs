use boxer::array::BoxerArrayF32;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use compositor::{Matrix, Scalar};

#[no_mangle]
pub fn compositor_matrix_new(values: *mut ValueBox<BoxerArrayF32>) -> *mut ValueBox<Matrix> {
    values.with_not_null_return(std::ptr::null_mut(), |values| {
        let buffer: &mut [f32; 9] = values.to_slice().try_into().unwrap();
        let buffer = &unsafe { *(buffer as *const [f32; 9] as *const [Scalar; 9]) };

        ValueBox::new(Matrix::from_9(buffer.clone())).into_raw()
    })
}

#[no_mangle]
pub fn compositor_matrix_drop(matrix: &mut *mut ValueBox<Matrix>) {
    matrix.drop();
}
