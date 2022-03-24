use boxer::{ValueBox, ValueBoxPointer};
use compositor::{Layer, OffsetLayer, Point};
use std::cell::RefCell;
use std::rc::Rc;

#[no_mangle]
pub fn skia_offset_layer_new_point(x: scalar, y: scalar) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> =
        Rc::new(RefCell::new(OffsetLayer::new(Point::new_f32(x, y))));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_offset_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> =
        Rc::new(RefCell::new(OffsetLayer::new(Point::new_f32(0.0, 0.0))));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_offset_layer_get_x(layer_ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>) -> scalar {
    layer_ptr.with_not_null_value_return(0.0, |layer| layer.borrow().offset.x)
}

#[no_mangle]
pub fn skia_offset_layer_get_y(layer_ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>) -> scalar {
    layer_ptr.with_not_null_value_return(0.0, |layer| layer.borrow().offset.y)
}

#[no_mangle]
pub fn skia_offset_layer_set_offset(
    _ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>,
    x: scalar,
    y: scalar,
) {
    _ptr.with_not_null_value(|layer| layer.borrow_mut().offset = Point::new_f32(x, y));
}
