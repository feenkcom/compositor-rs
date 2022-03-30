use boxer::{ValueBox, ValueBoxPointer};
use compositor::Path;
use compositor_skia::SkiaPath;

#[no_mangle]
pub fn skia_compositor_path_new(path: *mut ValueBox<compositor_skia::Path>) -> *mut ValueBox<Path> {
    path.with_not_null_value_return(std::ptr::null_mut(), |path| {
        ValueBox::new(Path::new(Box::new(SkiaPath::new(path)))).into_raw()
    })
}
