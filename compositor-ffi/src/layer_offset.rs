use compositor::{Layer, OffsetLayer, Point};
use std::sync::Arc;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_offset_layer_new() -> OwnedPtr<Arc<dyn Layer>> {
    OwnedPtr::new(Arc::new(OffsetLayer::new()) as Arc<dyn Layer>)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_offset_layer_new_point(
    x: f32,
    y: f32,
) -> OwnedPtr<Arc<dyn Layer>> {
    OwnedPtr::new(Arc::new(OffsetLayer::new_offset(Point::new_f32(x, y))) as Arc<dyn Layer>)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_offset_layer_with_point(
    layer: BorrowedPtr<Arc<dyn Layer>>,
    x: f32,
    y: f32,
) -> OwnedPtr<Arc<dyn Layer>> {
    layer
        .with_ref_ok(|layer| {
            let offset_layer = layer
                .any()
                .downcast_ref::<OffsetLayer>()
                .expect("Is not an offset layer!");
            OwnedPtr::new(Arc::new(offset_layer.with_offset(Point::new_f32(x, y)))
                as Arc<dyn Layer>)
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_offset_layer_get_x(layer: BorrowedPtr<Arc<dyn Layer>>) -> f32 {
    layer
        .with_ref_ok(|layer| {
            let offset_layer = layer
                .any()
                .downcast_ref::<OffsetLayer>()
                .expect("Is not an offset layer!");

            offset_layer.offset().x().into()
        })
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_offset_layer_get_y(layer: BorrowedPtr<Arc<dyn Layer>>) -> f32 {
    layer
        .with_ref_ok(|layer| {
            let offset_layer = layer
                .any()
                .downcast_ref::<OffsetLayer>()
                .expect("Is not an offset layer!");

            offset_layer.offset().y().into()
        })
        .or_log(0.0)
}
