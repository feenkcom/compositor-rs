use compositor::{Filter, Radius};
use value_box::OwnedPtr;

#[unsafe(no_mangle)]
pub extern "C" fn compositor_filter_blur_new(sigma_x: f32, sigma_y: f32) -> OwnedPtr<Filter> {
    OwnedPtr::new(Filter::blur(Radius::new(sigma_x, sigma_y)))
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_filter_drop(filter: OwnedPtr<Filter>) {
    drop(filter);
}
