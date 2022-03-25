use boxer::boxes::{ReferenceBox, ReferenceBoxPointer};
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use compositor::Layer;
use compositor_skia::{Canvas, ImageCache, ShadowCache, SkiaCompositor};
use std::sync::Arc;

#[no_mangle]
pub fn skia_compositor_compose(
    layer: *mut ValueBox<Arc<dyn Layer>>,
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

#[no_mangle]
pub fn skia_compositor_image_cache_new() -> *mut ValueBox<ImageCache> {
    ValueBox::new(ImageCache::new()).into_raw()
}

#[no_mangle]
pub fn skia_compositor_image_cache_drop(ptr: &mut *mut ValueBox<ImageCache>) {
    ptr.drop();
}

#[no_mangle]
pub fn skia_compositor_shadow_cache_new() -> *mut ValueBox<ShadowCache> {
    ValueBox::new(ShadowCache::new()).into_raw()
}

#[no_mangle]
pub fn skia_compositor_shadow_cache_drop(ptr: &mut *mut ValueBox<ShadowCache>) {
    ptr.drop();
}
