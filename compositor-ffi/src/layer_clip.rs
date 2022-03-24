use boxer::{ValueBox, ValueBoxPointer};
use compositor::{ClipLayer, Layer};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

#[no_mangle]
pub fn skia_clip_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(ClipLayer::new()));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_clip_layer_rect(
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    offset_x: scalar,
    offset_y: scalar,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(ClipLayer::rect(
        Rect::new(left, top, right, bottom),
        Vector::new(offset_x, offset_y),
    )));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_clip_layer_rrect(
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r_top_left: scalar,
    r_top_right: scalar,
    r_bottom_right: scalar,
    r_bottom_left: scalar,
    offset_x: scalar,
    offset_y: scalar,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(ClipLayer::rounded_rectangle(
        RRect::new_rect_radii(
            Rect::new(left, top, right, bottom),
            &[
                Vector::new(r_top_left, r_top_left),
                Vector::new(r_top_right, r_top_right),
                Vector::new(r_bottom_right, r_bottom_right),
                Vector::new(r_bottom_left, r_bottom_left),
            ],
        ),
        Vector::new(offset_x, offset_y),
    )));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_clip_layer_path(
    path_ptr: *mut ValueBox<Path>,
    offset_x: scalar,
    offset_y: scalar,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = path_ptr.with_value(
        || Rc::new(RefCell::new(ClipLayer::new())),
        |path| {
            Rc::new(RefCell::new(ClipLayer::path(
                path,
                Vector::new(offset_x, offset_y),
            )))
        },
    );
    ValueBox::new(layer).into_raw()
}
