use boxer::{ValueBox, ValueBoxPointer};
use compositor::Picture;
use compositor_skia::SkiaPicture;
use std::sync::Arc;

#[no_mangle]
pub fn skia_compositor_picture_new(
    picture: *mut ValueBox<compositor_skia::Picture>,
) -> *mut ValueBox<Arc<dyn Picture>> {
    picture.with_not_null_value_return(std::ptr::null_mut(), |picture| {
        ValueBox::new(Arc::new(SkiaPicture::new(picture)) as Arc<dyn Picture>).into_raw()
    })
}
