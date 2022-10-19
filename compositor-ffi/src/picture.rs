use std::sync::Arc;

use boxer::{ValueBox, ValueBoxPointer};

use compositor::Picture;

#[no_mangle]
pub fn compositor_picture_drop(picture: *mut ValueBox<Arc<dyn Picture>>) {
    picture.release();
}
