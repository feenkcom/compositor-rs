use boxer::{ValueBox, ValueBoxPointerReference};
use compositor::Picture;
use std::sync::Arc;

#[no_mangle]
pub fn compositor_picture_drop(picture: &mut *mut ValueBox<Arc<dyn Picture>>) {
    picture.drop()
}
