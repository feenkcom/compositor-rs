use compositor::Picture;
use compositor_skia::SkiaPicture;
use std::sync::Arc;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_compositor_picture_new(
    picture: *mut ValueBox<compositor_skia::Picture>,
) -> *mut ValueBox<Arc<dyn Picture>> {
    picture
        .to_ref()
        .map(|picture| Arc::new(SkiaPicture::new(picture.clone())) as Arc<dyn Picture>)
        .into_raw()
}
