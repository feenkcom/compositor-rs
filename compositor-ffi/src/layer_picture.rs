use compositor::{Layer, Picture, PictureLayer};
use std::sync::Arc;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_picture_layer_new(
    picture: BorrowedPtr<Arc<dyn Picture>>,
    needs_cache: bool,
) -> OwnedPtr<Arc<dyn Layer>> {
    picture
        .with_clone_ok(|picture| {
            OwnedPtr::new(Arc::new(PictureLayer::new(picture, needs_cache)) as Arc<dyn Layer>)
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_picture_layer_needs_cache(
    layer: BorrowedPtr<Arc<dyn Layer>>,
) -> bool {
    layer
        .with_ref_ok(|layer| {
            let picture_layer = layer
                .any()
                .downcast_ref::<PictureLayer>()
                .expect("Is not an offset layer!");

            picture_layer.needs_cache()
        })
        .or_log(false)
}
