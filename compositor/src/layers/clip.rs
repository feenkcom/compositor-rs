use std::sync::Arc;

use crate::{Compositor, Layer, Path, Point, Rectangle, RoundedRectangle};

#[derive(Debug, Clone, PartialEq)]
pub enum Clip {
    None,
    Rectangle(Rectangle),
    RoundedRectangle(RoundedRectangle),
    Path(Path),
}

#[derive(Debug, Clone)]
pub struct ClipLayer {
    layers: Vec<Arc<dyn Layer>>,
    offset: Point,
    clip: Clip,
}

impl ClipLayer {
    fn new() -> Self {
        Self {
            layers: vec![],
            offset: Point::zero(),
            clip: Clip::None,
        }
    }

    pub fn rectangle(rectangle: Rectangle, offset: Point) -> Self {
        let mut layer = Self::new();
        layer.set_offset(offset);
        layer.clip_rectangle(rectangle);
        layer
    }

    pub fn rounded_rectangle(rounded_rectangle: RoundedRectangle, offset: Point) -> Self {
        let mut layer = Self::new();
        layer.set_offset(offset);
        layer.clip_rounded_rectangle(rounded_rectangle);
        layer
    }

    pub fn path(path: Path, offset: Point) -> Self {
        let mut layer = Self::new();
        layer.set_offset(offset);
        layer.clip_path(path);
        layer
    }

    pub fn offset(&self) -> &Point {
        &self.offset
    }

    pub fn clip(&self) -> &Clip {
        &self.clip
    }

    fn set_offset(&mut self, offset: Point) {
        self.offset = offset;
    }

    fn clip_rectangle(&mut self, rectangle: Rectangle) {
        self.clip = Clip::Rectangle(rectangle);
    }

    fn clip_rounded_rectangle(&mut self, rounded_rectangle: RoundedRectangle) {
        self.clip = Clip::RoundedRectangle(rounded_rectangle);
    }

    fn clip_path(&mut self, path: Path) {
        self.clip = Clip::Path(path);
    }
}

impl Layer for ClipLayer {
    fn layers(&self) -> &[Arc<dyn Layer>] {
        self.layers.as_slice()
    }

    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_clip(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let clip = ClipLayer::new();
        assert_eq!(clip.clip, Clip::None);
        assert_eq!(clip.offset, Point::zero());
        assert_eq!(clip.layers.len(), 0);
    }

    #[test]
    fn test_new_dyn_object() {
        let layer: Arc<dyn Layer> = Arc::new(ClipLayer::new());
        assert_eq!(layer.count_layers(), 0);
    }
}
