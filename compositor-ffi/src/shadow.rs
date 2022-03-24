use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use compositor::{Color, Geometry, Layer, Point, Radius, Shadow, ShadowLayer};
use std::sync::Arc;

/// Creates a new shadow consuming the geometry
#[no_mangle]
pub fn compositor_shadow_new(
    argb: u32,
    sigma_x: f32,
    sigma_y: f32,
    delta_x: f32,
    delta_y: f32,
    mut geometry: *mut ValueBox<Geometry>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    geometry.with_not_null_value_consumed_return(std::ptr::null_mut(), |geometry| {
        let shadow = Shadow::new(
            Color::from_argb(argb),
            Radius::new(sigma_x, sigma_y),
            Point::new_f32(delta_x, delta_y),
            geometry,
        );
        ValueBox::new(Arc::new(ShadowLayer::new(shadow)) as Arc<dyn Layer>).into_raw()
    })
}

#[no_mangle]
pub fn compositor_shadow_drop(shadow: &mut *mut ValueBox<Shadow>) {
    shadow.drop()
}
