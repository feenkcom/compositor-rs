#![cfg_attr(feature = "phlow", feature(min_specialization))]

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate lazy_static;

pub use skia_safe::{Canvas, Path, Picture};

pub use cache::Cache;
pub use image_cache::ImageCache;
pub use renderers::*;
pub use shadow_cache::ShadowCache;
pub use skia_cacheless_compositor::SkiaCachelessCompositor;
pub use skia_compositor::SkiaCompositor;
pub use types::*;

mod cache;
mod image_cache;
mod platform_compositor;
mod renderers;
mod shadow_cache;
mod skia_cacheless_compositor;
mod skia_compositor;
mod textures;
mod types;
mod utils;

cfg_if! {
    if #[cfg(feature = "phlow")] {
        use phlow::{define_extensions, import_extensions};
        use phlow_extensions::CoreExtensions;
        use compositor::CompositorExtensions;

        mod extensions;

        define_extensions!(CompositorSkiaExtensions);
        import_extensions!(CoreExtensions, CompositorExtensions, CompositorSkiaExtensions);
    }
}
