use std::sync::Arc;
use value_box::{ValueBox, ValueBoxPointer};

use compositor::Picture;

#[unsafe(no_mangle)]
pub extern "C" fn compositor_picture_drop(picture: *mut ValueBox<Arc<dyn Picture>>) {
    picture.release();
}
