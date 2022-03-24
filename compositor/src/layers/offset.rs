use crate::{Compositor, Layer, Point};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OffsetLayer {
    layers: Vec<Arc<dyn Layer>>,
    offset: Point,
}

impl OffsetLayer {
    pub fn new() -> Self {
        Self::new_offset(Point::zero())
    }

    pub fn new_offset(offset: Point) -> Self {
        Self {
            layers: vec![],
            offset,
        }
    }

    pub fn offset(&self) -> &Point {
        &self.offset
    }

    /// Create a new offset layer with a given offset preserving the sub-layers
    pub fn with_offset(&self, offset: Point) -> Self {
        Self {
            layers: self.layers.clone(),
            offset,
        }
    }
}

impl Layer for OffsetLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_offset(self);
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
        let offset = OffsetLayer::new();
        assert_eq!(offset.offset, Point::zero());
        assert_eq!(offset.count_layers(), 0);
    }

    #[test]
    fn test_new_offset() {
        let offset = OffsetLayer::new_offset(Point::new_f32(20.0, 10.0));
        assert_eq!(offset.offset, Point::new_f32(20.0, 10.0));
        assert_eq!(offset.count_layers(), 0);
    }

    #[test]
    fn test_with_offset() {
        let offset = OffsetLayer::new().with_offset(Point::new_f32(10.0, 20.0));
        assert_eq!(offset.offset, Point::new_f32(10.0, 20.0));
    }

    #[test]
    fn test_arc_with_offset() {
        let layer = Arc::new(OffsetLayer::new());
        let layer_with_offset = layer.with_offset(Point::new_f32(10.0, 20.0));
        assert_eq!(layer_with_offset.offset, Point::new_f32(10.0, 20.0));
    }

    #[test]
    fn test_new_dyn_object() {
        let layer: Arc<dyn Layer> = Arc::new(OffsetLayer::new());
        assert_eq!(layer.count_layers(), 0);
    }
}
