use boxer::boxes::{ReferenceBox, ReferenceBoxPointer};
use boxer::{ValueBox, ValueBoxPointer};
use compositor::Layer;
use compositor_skia::{Canvas, ImageCache, ShadowCache, SkiaCompositor};
use std::rc::Rc;

#[no_mangle]
pub fn skia_compositor_compose(
    layer: *mut ValueBox<Rc<dyn Layer>>,
    canvas: *mut ReferenceBox<Canvas>,
    image_cache: *mut ValueBox<ImageCache>,
    shadow_cache: *mut ValueBox<ShadowCache>,
) {
    layer.with_not_null(|layer| {
        canvas.with_not_null(|canvas| {
            image_cache.with_not_null(|image_cache| {
                shadow_cache.with_not_null(|shadow_cache| {
                    let mut compositor = SkiaCompositor::new(canvas, image_cache, shadow_cache);
                    layer.compose(&mut compositor);
                })
            })
        })
    })
}
