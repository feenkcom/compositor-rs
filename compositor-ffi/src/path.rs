use boxer::{ValueBox, ValueBoxPointer};
use compositor::Path;

#[no_mangle]
pub fn compositor_path_drop(path: *mut ValueBox<Path>) {
    path.release();
}
