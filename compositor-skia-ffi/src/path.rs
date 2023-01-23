use compositor::Path;
use compositor_skia::SkiaPath;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_compositor_path_new(path: *mut ValueBox<compositor_skia::Path>) -> *mut ValueBox<Path> {
    path.with_clone_ok(|path| Path::new(Box::new(SkiaPath::new(path))))
        .into_raw()
}
