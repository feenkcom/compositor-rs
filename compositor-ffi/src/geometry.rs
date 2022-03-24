use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use compositor::{Geometry, Path, Rectangle};

#[no_mangle]
pub fn compositor_geometry_none() -> *mut ValueBox<Geometry> {
    ValueBox::new(Geometry::None).into_raw()
}

#[no_mangle]
pub fn compositor_geometry_new_rectangle(
    left: f32,
    top: f32,
    width: f32,
    height: f32,
) -> *mut ValueBox<Geometry> {
    ValueBox::new(Geometry::Rectangle(Rectangle::new(
        left, top, width, height,
    )))
    .into_raw()
}

/// Creates a new geometry from a given path consuming that path
#[no_mangle]
pub fn compositor_geometry_new_path(mut path: *mut ValueBox<Path>) -> *mut ValueBox<Geometry> {
    path.with_not_null_value_consumed_return(std::ptr::null_mut(), |path| {
        ValueBox::new(Geometry::Path(path)).into_raw()
    })
}

#[no_mangle]
pub fn compositor_geometry_drop(path: &mut *mut ValueBox<Geometry>) {
    path.drop();
}

// #[no_mangle]
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
