use std::sync::Arc;

use value_box::{ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

use compositor::Picture;
use compositor_skia::SkiaPicture;

#[no_mangle]
pub fn skia_compositor_picture_new(
    picture: *mut ValueBox<compositor_skia::Picture>,
) -> *mut ValueBox<Arc<dyn Picture>> {
    picture
        .with_clone_ok(|picture| {
            ValueBox::new(Arc::new(SkiaPicture::new(picture)) as Arc<dyn Picture>)
        })
        .into_raw()
}
