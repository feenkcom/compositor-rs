use crate::{Compositor, Layer};
use compositor_texture::TextureDesc;
use std::any::Any;
use std::ffi::c_void;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TextureLayer {
    width: u32,
    height: u32,
    texture: Texture,
}

impl TextureLayer {
    pub fn new(width: u32, height: u32, texture: Texture) -> Self {
        Self {
            texture,
            width,
            height,
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
    Borrowed(BorrowedTexture),
    External(ExternalTexture),
}

/// Allows users to draw into a borrowed texture which is managed by the compositor
#[derive(Clone)]
pub struct BorrowedTexture {
    pub rendering: Arc<dyn Fn(TextureDesc, *const c_void) + Send + Sync>,
    pub payload: *const c_void,
}

unsafe impl Sync for BorrowedTexture {}
unsafe impl Send for BorrowedTexture {}

impl Debug for BorrowedTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OwnedTexture")
    }
}

#[derive(Clone, Debug)]
pub enum ExternalTexture {}
