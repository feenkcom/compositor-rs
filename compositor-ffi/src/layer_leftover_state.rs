
#[no_mangle]
pub fn skia_leftover_state_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(LeftoverStateLayer::new()));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_leftover_state_layer_clip_rect(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        layer.borrow_mut().clip_rect(
            Rect::new(left, top, right, bottom),
            Vector::new(offset_x, offset_y),
        );
    })
}

#[no_mangle]
pub fn skia_leftover_state_layer_clip_rrect(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
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
) {
    _ptr.with_not_null_value(|layer| {
        layer.borrow_mut().clip_rrect(
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
        );
    })
}

#[no_mangle]
pub fn skia_leftover_state_layer_clip_path(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    _ptr_path: *mut ValueBox<Path>,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        _ptr_path.with_not_null_value(|path| {
            layer
                .borrow_mut()
                .clip_path(path, Vector::new(offset_x, offset_y));
        })
    })
}

#[no_mangle]
pub fn skia_leftover_state_layer_transform(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    scale_x: scalar,
    skew_x: scalar,
    trans_x: scalar,
    skew_y: scalar,
    scale_y: scalar,
    trans_y: scalar,
    persp_0: scalar,
    persp_1: scalar,
    persp_2: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        layer.borrow_mut().transform(
            Matrix::new_all(
                scale_x, skew_x, trans_x, skew_y, scale_y, trans_y, persp_0, persp_1, persp_2,
            ),
            Vector::new(offset_x, offset_y),
        );
    })
}
