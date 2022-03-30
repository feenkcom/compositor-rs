use boxer::{ValueBox, ValueBoxPointerReference};
use compositor::Path;

#[no_mangle]
pub fn compositor_path_drop(path: &mut *mut ValueBox<Path>) {
    path.drop()
}
