use std::sync::Arc;
use value_box::OwnedPtr;

use compositor::Picture;

#[unsafe(no_mangle)]
pub extern "C" fn compositor_picture_drop(picture: OwnedPtr<Arc<dyn Picture>>) {
    drop(picture);
}
