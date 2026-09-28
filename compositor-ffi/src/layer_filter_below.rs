use std::sync::Arc;

use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

use compositor::{Filter, FilterBelowLayer, Geometry, Layer, Point};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_filter_below_layer_new(
    filter: BorrowedPtr<Filter>,
    geometry: BorrowedPtr<Geometry>,
    offset_x: f32,
    offset_y: f32,
) -> OwnedPtr<Arc<dyn Layer>> {
    filter
        .with_clone_ok(|filter| {
            geometry
                .with_clone_ok(|geometry| {
                    let offset = Point::new_f32(offset_x, offset_y);
                    OwnedPtr::new(
                        Arc::new(FilterBelowLayer::new(filter, geometry, offset)) as Arc<dyn Layer>
                    )
                })
                .or_log(OwnedPtr::null())
        })
        .or_log(OwnedPtr::null())
}
