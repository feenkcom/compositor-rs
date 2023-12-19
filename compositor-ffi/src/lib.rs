use phlow::import_extensions;
use phlow_extensions::CoreExtensions;

use compositor::CompositorExtensions;
pub use geometry::*;
pub use layer::*;
pub use layer_clip::*;
pub use layer_leftover::*;
pub use layer_offset::*;
pub use layer_picture::*;
pub use layer_shadow::*;
pub use layer_tiled::*;
pub use layer_transformation::*;
pub use matrix::*;
pub use picture::*;
pub use shadow::*;

mod geometry;
mod layer;
mod layer_clip;
mod layer_leftover;
mod layer_offset;
mod layer_picture;
mod layer_shadow;
mod layer_tiled;
mod layer_transformation;
mod matrix;
mod path;
mod picture;
mod shadow;

import_extensions!(CompositorExtensions, CoreExtensions);
