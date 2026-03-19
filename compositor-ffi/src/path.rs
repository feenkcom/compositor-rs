use compositor::Path;
use value_box::OwnedPtr;

#[unsafe(no_mangle)]
pub extern "C" fn compositor_path_drop(path: OwnedPtr<Path>) {
    drop(path);
}
