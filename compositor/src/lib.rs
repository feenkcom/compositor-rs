#![cfg_attr(feature = "phlow", feature(min_specialization))]

#[macro_use]
extern crate cfg_if;

mod compositor;
mod layers;
mod types;

pub use crate::compositor::Compositor;
pub use layers::*;
pub use types::*;

cfg_if! {
    if #[cfg(feature = "phlow")] {
        use phlow::{define_extensions, import_extensions};
        use phlow_extensions::CoreExtensions;

        define_extensions!(CompositorExtensions);
        import_extensions!(CoreExtensions, CompositorExtensions);
    }
}
