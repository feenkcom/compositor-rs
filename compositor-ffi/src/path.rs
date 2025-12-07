use compositor::Path;
use value_box::{ValueBox, ValueBoxPointer};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_path_drop(path: *mut ValueBox<Path>) {
    path.release();
}
