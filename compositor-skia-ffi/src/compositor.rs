use std::sync::Arc;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

use compositor::{Compositor, Layer};
use compositor_skia::{Cache, Canvas, SkiaCachelessCompositor, SkiaCompositor};

#[unsafe(no_mangle)]
pub fn skia_compositor_compose(
    layer: BorrowedPtr<Arc<dyn Layer>>,
    canvas: BorrowedPtr<Canvas>,
    mut cache: BorrowedPtr<Cache>,
) {
    layer
        .with_ref(|layer| {
            cache.with_mut_ok(|cache| {
                canvas.with_ref_ok(|canvas| {
                    SkiaCompositor::new(None, canvas, cache).compose(layer.clone());
                })
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub fn skia_cacheless_compositor_compose(
    layer: BorrowedPtr<Arc<dyn Layer>>,
    canvas: BorrowedPtr<Canvas>,
) {
    layer
        .with_ref_ok(|layer| {
            canvas.with_ref_ok(|canvas| {
                SkiaCachelessCompositor::new(canvas).compose(layer.clone());
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub fn skia_compositor_cache_new() -> OwnedPtr<Cache> {
    OwnedPtr::new(Cache::new())
}

#[unsafe(no_mangle)]
pub fn skia_compositor_cache_drop(cache: OwnedPtr<Cache>) {
    drop(cache);
}
