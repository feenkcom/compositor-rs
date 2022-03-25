use crate::{Compositor, Layer, Rectangle};
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PictureLayer {
    picture: Arc<dyn Picture>,
    picture_id: u32,
    needs_cache: bool,
}

impl PictureLayer {
    pub fn new(picture: Arc<dyn Picture>, needs_cache: bool) -> Self {
        let id = picture.unique_id();

        Self {
            picture,
            picture_id: id,
            needs_cache,
        }
    }

    pub fn id(&self) -> u32 {
        self.picture_id
    }

    pub fn picture(&self) -> &Arc<dyn Picture> {
        &self.picture
    }

    pub fn needs_cache(&self) -> bool {
        self.needs_cache
    }

    pub fn cull_rect(&self) -> Rectangle {
        self.picture.cull_rect()
    }
}

impl Layer for PictureLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_picture(self);
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

pub trait Picture: Debug {
    fn unique_id(&self) -> u32;
    fn cull_rect(&self) -> Rectangle;
    fn any(&self) -> &dyn Any;
}
