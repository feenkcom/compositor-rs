use crate::{Compositor, Layer, Matrix};
use std::sync::Arc;

#[derive(Debug)]
pub struct TransformationLayer {
    layers: Vec<Arc<dyn Layer>>,
    matrix: Matrix,
}

impl TransformationLayer {
    pub fn new(matrix: Matrix) -> Self {
        Self {
            layers: vec![],
            matrix,
        }
    }

    pub fn matrix(&self) -> &Matrix {
        &self.matrix
    }

    pub fn with_layer(&self, layer: Arc<dyn Layer>) -> Self {
        let mut new_layers = self.layers.clone();
        new_layers.push(layer);
        Self {
            layers: new_layers,
            matrix: self.matrix.clone(),
        }
    }
}

impl Layer for TransformationLayer {
    fn layers(&self) -> &[Arc<dyn Layer>] {
        self.layers.as_slice()
    }

    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_transformation(self);
    }
}
