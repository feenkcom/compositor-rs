use compositor::Path;
use value_box::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn compositor_path_drop(path: *mut ValueBox<Path>) {
    path.release();
}
