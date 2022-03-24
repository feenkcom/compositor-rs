use std::any::Any;
use std::sync::Arc;

use crate::{Compositor, Geometry, Layer, Point};

#[derive(Debug, Clone)]
pub struct ClipLayer {
    layers: Vec<Arc<dyn Layer>>,
    offset: Point,
    geometry: Geometry,
}

impl ClipLayer {
    pub fn none() -> Self {
        Self {
            layers: vec![],
            offset: Point::zero(),
            geometry: Geometry::None,
        }
    }

    pub fn new(geometry: Geometry, offset: Point) -> Self {
        Self {
            layers: vec![],
            offset,
            geometry,
        }
    }

    pub fn offset(&self) -> &Point {
        &self.offset
    }

    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }
}

impl Layer for ClipLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_clip(self)
    }

    fn layers(&self) -> &[Arc<dyn Layer>] {
        self.layers.as_slice()
    }

    fn clone_arc(&self) -> Arc<dyn Layer> {
        Arc::new(self.clone())
    }

    fn any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let clip = ClipLayer::none();
        assert_eq!(clip.geometry, Geometry::None);
        assert_eq!(clip.offset, Point::zero());
        assert_eq!(clip.layers.len(), 0);
    }

    #[test]
    fn test_new_dyn_object() {
        let layer: Arc<dyn Layer> = Arc::new(ClipLayer::none());
        assert_eq!(layer.count_layers(), 0);
    }
}
