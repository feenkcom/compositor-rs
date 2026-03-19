use compositor::Path;
use compositor_skia::SkiaPath;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub fn skia_compositor_path_new(path: BorrowedPtr<compositor_skia::Path>) -> OwnedPtr<Path> {
    path.with_clone_ok(|path| OwnedPtr::new(Path::new(Box::new(SkiaPath::new(path)))))
        .or_log(OwnedPtr::null())
}
