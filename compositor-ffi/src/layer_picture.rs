#[no_mangle]
pub fn skia_picture_layer_new_picture(
    picture_ptr: *mut ValueBox<Picture>,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    picture_ptr.with_value(
        || {
            let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(PictureLayer::from_picture(
                Picture::new_placeholder(Rect::new(0.0, 0.0, 50.0, 50.0)),
            )));
            ValueBox::new(layer).into_raw()
        },
        |picture| {
            let layer: Rc<RefCell<dyn Layer>> =
                Rc::new(RefCell::new(PictureLayer::from_picture(picture)));
            ValueBox::new(layer).into_raw()
        },
    )
}

#[no_mangle]
pub fn skia_picture_layer_get_needs_cache(
    layer_ptr: *mut ValueBox<Rc<RefCell<PictureLayer>>>,
) -> bool {
    layer_ptr.with_not_null_value_return(false, |layer| layer.borrow().needs_cache)
}

#[no_mangle]
pub fn skia_picture_layer_set_needs_cache(
    layer_ptr: *mut ValueBox<Rc<RefCell<PictureLayer>>>,
    needs_cache: bool,
) {
    layer_ptr.with_not_null_value(|layer| layer.borrow_mut().needs_cache = needs_cache);
}
