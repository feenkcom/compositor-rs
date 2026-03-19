use std::sync::Arc;

use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

use compositor::Picture;
use compositor_skia::SkiaPicture;

#[unsafe(no_mangle)]
pub fn skia_compositor_picture_new(
    picture: BorrowedPtr<compositor_skia::Picture>,
) -> OwnedPtr<Arc<dyn Picture>> {
    picture
        .with_clone_ok(|picture| {
            OwnedPtr::new(Arc::new(SkiaPicture::new(picture)) as Arc<dyn Picture>)
        })
        .or_log(OwnedPtr::null())
}
