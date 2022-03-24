use boxer::ValueBox;
use compositor::{Layer, ShadowLayer};
use std::cell::RefCell;
use std::rc::Rc;

#[no_mangle]
pub fn skia_shadow_layer_new(
    delta_x: scalar,
    delta_y: scalar,
    sigma_x: scalar,
    sigma_y: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(ShadowLayer::new(
        Color::from_argb(a, r, g, b),
        (sigma_x, sigma_y),
        Vector::new(delta_x, delta_y),
        Path::new(),
    )));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_shadow_layer_set_path(
    _ptr: *mut ValueBox<Rc<RefCell<ShadowLayer>>>,
    _path_ptr: *mut ValueBox<Path>,
) {
    _ptr.with_not_null_value(|layer| {
        _path_ptr.with_not_null_value(|path| layer.borrow_mut().set_path(path))
    });
}

#[no_mangle]
pub fn skia_shadow_layer_set_rectangle(
    _ptr: *mut ValueBox<Rc<RefCell<ShadowLayer>>>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        let mut path = Path::new();
        path.add_rect(Rect::new(left, top, right, bottom), None);
        layer.borrow_mut().set_path(path)
    })
}

#[no_mangle]
pub fn skia_shadow_layer_set_rounded_rectangle(
    _ptr: *mut ValueBox<Rc<RefCell<ShadowLayer>>>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r_left: scalar,
    r_top: scalar,
    r_right: scalar,
    r_bottom: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        let mut path = Path::new();
        let rect = Rect::new(left, top, right, bottom);
        let radii = [
            Vector::new(r_left, r_left),
            Vector::new(r_top, r_top),
            Vector::new(r_right, r_right),
            Vector::new(r_bottom, r_bottom),
        ];
        path.add_rrect(&RRect::new_rect_radii(rect, &radii), None);
        layer.borrow_mut().set_path(path)
    })
}

#[no_mangle]
pub fn skia_shadow_layer_set_circle(
    _ptr: *mut ValueBox<Rc<RefCell<ShadowLayer>>>,
    origin_x: scalar,
    origin_y: scalar,
    radius: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        let mut path = Path::new();
        path.add_circle(Point::new(origin_x, origin_y), radius, None);
        layer.borrow_mut().set_path(path)
    })
}
