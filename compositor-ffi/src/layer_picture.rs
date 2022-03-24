use boxer::{ValueBox, ValueBoxPointer};
use compositor::{Layer, Picture, PictureLayer};
use std::sync::Arc;

#[no_mangle]
pub fn compositor_picture_layer_new(
    picture: *mut ValueBox<Arc<dyn Picture>>,
    needs_cache: bool,
) -> *mut ValueBox<Arc<dyn Layer>> {
    picture.with_not_null_value_return(std::ptr::null_mut(), |picture| {
        ValueBox::new(Arc::new(PictureLayer::new(picture, needs_cache)) as Arc<dyn Layer>)
            .into_raw()
    })
}

#[no_mangle]
pub fn compositor_picture_layer_needs_cache(layer: *mut ValueBox<Arc<PictureLayer>>) -> bool {
    layer.with_not_null_value_return(false, |layer| {
        let picture_layer = layer
            .any()
            .downcast_ref::<PictureLayer>()
            .expect("Is not an offset layer!");

        picture_layer.needs_cache()
    })
}
