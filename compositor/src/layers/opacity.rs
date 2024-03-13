use crate::{Compositor, Layer};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OpacityLayer {
    layers: Vec<Arc<dyn Layer>>,
    alpha: f32,
}

impl OpacityLayer {
    pub fn new() -> Self {
        Self::new_alpha(1.0)
    }

    pub fn new_alpha(alpha: f32) -> Self {
        Self {
            layers: vec![],
            alpha,
        }
    }

    pub fn wrap_with_alpha(layer: impl Layer, alpha: f32) -> Self {
        Self {
            layers: vec![layer.clone_arc()],
            alpha,
        }
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    /// Create a new offset layer with a given offset preserving the sub-layers
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self {
            layers: self.layers.clone(),
            alpha,
        }
    }
}

impl Layer for OpacityLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_opacity(self);
    }

    fn layers(&self) -> &[Arc<dyn Layer>] {
        self.layers.as_slice()
    }

    fn with_layers(&self, layers: Vec<Arc<dyn Layer>>) -> Arc<dyn Layer> {
        Arc::new(Self {
            layers,
            alpha: self.alpha,
        })
    }

    fn clone_arc(&self) -> Arc<dyn Layer> {
        Arc::new(self.clone())
    }

    fn any(&self) -> &dyn Any {
        self
    }
}
