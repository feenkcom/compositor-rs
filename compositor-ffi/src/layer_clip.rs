use std::sync::Arc;

use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

use compositor::{ClipLayer, Geometry, Layer, Point};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_clip_layer_none() -> OwnedPtr<Arc<dyn Layer>> {
    OwnedPtr::new(Arc::new(ClipLayer::none()) as Arc<dyn Layer>)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_clip_layer_new(
    geometry: BorrowedPtr<Geometry>,
    offset_x: f32,
    offset_y: f32,
) -> OwnedPtr<Arc<dyn Layer>> {
    geometry
        .with_clone_ok(|geometry| {
            let layer = ClipLayer::new(geometry, Point::new_f32(offset_x, offset_y));
            OwnedPtr::new(Arc::new(layer) as Arc<dyn Layer>)
        })
        .or_log(OwnedPtr::null())
}
