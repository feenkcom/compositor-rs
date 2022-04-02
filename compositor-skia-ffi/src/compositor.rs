use boxer::boxes::{ReferenceBox, ReferenceBoxPointer};
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use compositor::{Compositor, Layer};
use compositor_skia::{Cache, Canvas, ImageCache, ShadowCache, SkiaCompositor};
use std::sync::Arc;

#[no_mangle]
pub fn skia_compositor_compose(
    layer: *mut ValueBox<Arc<dyn Layer>>,
    canvas: *mut ReferenceBox<Canvas>,
    cache: *mut ValueBox<Cache>,
) {
    layer.with_not_null(|layer| {
        canvas.with_not_null(|canvas| {
            cache.with_not_null(|cache| {
                SkiaCompositor::new(canvas, cache).compose(layer.clone());
            })
        })
    })
}

#[no_mangle]
pub fn skia_compositor_cache_new() -> *mut ValueBox<Cache> {
    ValueBox::new(Cache::new()).into_raw()
}

#[no_mangle]
pub fn skia_compositor_cache_drop(ptr: &mut *mut ValueBox<Cache>) {
    ptr.drop();
}
