use compositor::{Color, Geometry, Point, Radius, Shadow};
use value_box::{ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

/// Creates a new shadow consuming the geometry
#[no_mangle]
pub fn compositor_shadow_new(
    argb: u32,
    sigma_x: f32,
    sigma_y: f32,
    delta_x: f32,
    delta_y: f32,
    geometry: *mut ValueBox<Geometry>,
) -> *mut ValueBox<Shadow> {
    geometry
        .take_value()
        .map(|geometry| {
            let shadow = Shadow::new(
                Color::from_argb(argb),
                Radius::new(sigma_x, sigma_y),
                Point::new_f32(delta_x, delta_y),
                geometry,
            );
            ValueBox::new(shadow)
        })
        .into_raw()
}

#[no_mangle]
pub fn compositor_shadow_drop(shadow: *mut ValueBox<Shadow>) {
    shadow.release();
}
