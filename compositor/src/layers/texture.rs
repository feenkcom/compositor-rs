use std::any::Any;
use std::sync::Arc;

use crate::{Compositor, Layer};

#[derive(Debug, Clone)]
pub struct TextureLayer {
    texture: Texture,
    width: u32,
    height: u32,
    is_2d: bool,
}

impl TextureLayer {
    pub fn new(texture: Texture, width: u32, height: u32) -> Self {
        Self {
            texture,
            width,
            height,
            is_2d: true,
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Layer for TextureLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_texture(self)
    }

    fn layers(&self) -> &[Arc<dyn Layer>] {
        &[]
    }

    fn with_layers(&self, _layers: Vec<Arc<dyn Layer>>) -> Arc<dyn Layer> {
        self.clone_arc()
    }

    fn clone_arc(&self) -> Arc<dyn Layer> {
        Arc::new(self.clone())
    }

    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub enum Texture {
    #[cfg(target_os = "macos")]
    Metal(metal::Texture),
    Unsupported
}
