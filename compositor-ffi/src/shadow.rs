use compositor::{Color, Geometry, Point, Radius, Shadow};
use value_box::{OwnedPtr, ReturnBoxerResult};

/// Creates a new shadow consuming the geometry
#[unsafe(no_mangle)]
pub extern "C" fn compositor_shadow_new(
    argb: u32,
    sigma_x: f32,
    sigma_y: f32,
    delta_x: f32,
    delta_y: f32,
    geometry: OwnedPtr<Geometry>,
) -> OwnedPtr<Shadow> {
    geometry
        .with_value_ok(|geometry| {
            let shadow = Shadow::new(
                Color::from_argb(argb),
                Radius::new(sigma_x, sigma_y),
                Point::new_f32(delta_x, delta_y),
                geometry,
            );
            OwnedPtr::new(shadow)
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_shadow_drop(shadow: OwnedPtr<Shadow>) {
    drop(shadow);
}
