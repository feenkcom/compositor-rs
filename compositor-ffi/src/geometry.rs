use value_box::{OwnedPtr, ReturnBoxerResult};

use compositor::{Circle, Geometry, Path, Point, Radius, Rectangle, RoundedRectangle};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_geometry_none() -> OwnedPtr<Geometry> {
    OwnedPtr::new(Geometry::None)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_geometry_new_rectangle(
    left: f32,
    top: f32,
    width: f32,
    height: f32,
) -> OwnedPtr<Geometry> {
    OwnedPtr::new(Geometry::Rectangle(Rectangle::new(
        left, top, width, height,
    )))
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_geometry_new_rounded_rectangle(
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    r_left_x: f32,
    r_left_y: f32,
    r_top_x: f32,
    r_top_y: f32,
    r_right_x: f32,
    r_right_y: f32,
    r_bottom_x: f32,
    r_bottom_y: f32,
) -> OwnedPtr<Geometry> {
    OwnedPtr::new(Geometry::RoundedRectangle(RoundedRectangle::new(
        Rectangle::new(left, top, width, height),
        Radius::new(r_left_x, r_left_y),
        Radius::new(r_top_x, r_top_y),
        Radius::new(r_right_x, r_right_y),
        Radius::new(r_bottom_x, r_bottom_y),
    )))
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_geometry_new_circle(
    center_x: f32,
    center_y: f32,
    radius: f32,
) -> OwnedPtr<Geometry> {
    OwnedPtr::new(Geometry::Circle(Circle::new(
        Point::new(center_x, center_y),
        radius,
    )))
}

/// Creates a new geometry from a given path consuming that path
#[unsafe(no_mangle)]
pub extern "C" fn compositor_geometry_new_path(
    path: OwnedPtr<Path>,
) -> OwnedPtr<Geometry> {
    path.with_value_ok(|path| OwnedPtr::new(Geometry::Path(path)))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_geometry_drop(path: OwnedPtr<Geometry>) {
    drop(path);
}

// #[unsafe(no_mangle)]
// pub fn skia_clip_layer_rrect(
//     left: scalar,
//     top: scalar,
//     right: scalar,
//     bottom: scalar,
//     r_top_left: scalar,
//     r_top_right: scalar,
//     r_bottom_right: scalar,
//     r_bottom_left: scalar,
//     offset_x: scalar,
//     offset_y: scalar,
// ) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
//     let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(ClipLayer::rounded_rectangle(
//         RRect::new_rect_radii(
//             Rect::new(left, top, right, bottom),
//             &[
//                 Vector::new(r_top_left, r_top_left),
//                 Vector::new(r_top_right, r_top_right),
//                 Vector::new(r_bottom_right, r_bottom_right),
//                 Vector::new(r_bottom_left, r_bottom_left),
//             ],
//         ),
//         Vector::new(offset_x, offset_y),
//     )));
//     ValueBox::new(layer).into_raw()
// }
