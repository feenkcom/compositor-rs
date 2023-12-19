use crate::{Color, Compositor, Geometry, Layer, Point, Radius, Rectangle};
use std::any::Any;

use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ShadowLayer {
    layers: Vec<Arc<dyn Layer>>,
    shadow: Shadow,
}

impl ShadowLayer {
    pub fn new(shadow: Shadow) -> Self {
        Self {
            layers: vec![],
            shadow,
        }
    }

    pub fn shadow(&self) -> &Shadow {
        &self.shadow
    }
}

impl Layer for ShadowLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_shadow(self);
    }

    fn layers(&self) -> &[Arc<dyn Layer>] {
        self.layers.as_slice()
    }

    fn with_layers(&self, layers: Vec<Arc<dyn Layer>>) -> Arc<dyn Layer> {
        Arc::new(Self {
            layers,
            shadow: self.shadow.clone(),
        })
    }

    fn clone_arc(&self) -> Arc<dyn Layer> {
        Arc::new(self.clone())
    }

    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Shadow {
    color: Color,
    radius: Radius,
    offset: Point,
    geometry: Geometry,
    hash: Option<u64>,
}

impl Shadow {
    pub fn new(color: Color, radius: Radius, offset: Point, geometry: Geometry) -> Self {
        let mut shadow =
            Self {
                color,
                radius,
                offset,
                geometry,
                hash: None,
            };
        shadow.hash = Some(shadow.compute_default_hash());
        shadow
    }

    pub fn offset(&self) -> &Point {
        &self.offset
    }

    pub fn radius(&self) -> &Radius {
        &self.radius
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }

    pub fn compute_default_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn inflation_offset(&self) -> Point {
        let inflation_x = self.radius.width() * 3.0;
        let inflation_y = self.radius.height() * 3.0;
        Point::new_f32(inflation_x.into(), inflation_y.into())
    }

    pub fn total_offset(&self) -> Point {
        self.offset.clone() + self.inflation_offset()
    }

    pub fn cull_rect(&self) -> Rectangle {
        let bounds = self.geometry.bounds();
        let inflation = self.inflation_offset();

        bounds
            .inflate(inflation.x(), inflation.y())
            .translate(&self.offset)
            .translate(&inflation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_shadow_equals() {
        let shadow = Shadow::new(
            Color::from_argb(0),
            Radius::new(5.0, 5.0),
            Point::new_f32(0.0, 1.0),
            Geometry::None,
        );
        let similar_shadow = Shadow::new(
            Color::from_argb(0),
            Radius::new(5.0, 5.0),
            Point::new_f32(0.0, 1.0),
            Geometry::None,
        );

        assert_eq!(shadow, similar_shadow);
    }

    #[test]
    pub fn test_shadow_cull_rect() {
        let shadow = Shadow::new(
            Color::from_argb(0),
            Radius::new(20.0, 10.0),
            Point::new_f32(200.0, 100.0),
            Geometry::Rectangle(Rectangle::extent(300.0, 200.0)),
        );

        assert_eq!(
            shadow.cull_rect(),
            Rectangle::new(200.0, 100.0, 420.0, 260.0)
        );
    }
}
