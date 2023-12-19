use compositor::{Layer, Picture, PictureLayer};
use std::sync::Arc;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn compositor_picture_layer_new(
    picture: *mut ValueBox<Arc<dyn Picture>>,
    needs_cache: bool,
) -> *mut ValueBox<Arc<dyn Layer>> {
    picture
        .with_clone_ok(|picture| {
            ValueBox::new(Arc::new(PictureLayer::new(picture, needs_cache))
                as Arc<dyn Layer>)
        })
        .into_raw()
}

#[no_mangle]
pub fn compositor_picture_layer_needs_cache(layer: *mut ValueBox<Arc<dyn Layer>>) -> bool {
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
