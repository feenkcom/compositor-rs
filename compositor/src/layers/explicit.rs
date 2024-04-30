use crate::{Compositor, Drawable, Layer};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ExplicitLayer {
    drawable: Arc<dyn Drawable>,
}

impl ExplicitLayer {
    pub fn new(drawable: impl Drawable + 'static) -> Self {
        Self {
            drawable: Arc::new(drawable),
        }
    }

    pub fn drawable(&self) -> &Arc<dyn Drawable> {
        &self.drawable
    }
}

impl Layer for ExplicitLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_explicit(self)
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
