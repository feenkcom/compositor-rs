use crate::{Compositor, Geometry, Layer, Point, Radius};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FilterBelowLayer {
    layers: Vec<Arc<dyn Layer>>,
    filter: Filter,
    geometry: Geometry,
    offset: Point,
}

impl FilterBelowLayer {
    pub fn new(filter: Filter, geometry: Geometry, offset: Point) -> Self {
        Self {
            layers: vec![],
            filter,
            geometry,
            offset,
        }
    }

    pub fn filter(&self) -> &Filter {
        &self.filter
    }

    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }

    pub fn offset(&self) -> &Point {
        &self.offset
    }
}

impl Layer for FilterBelowLayer {
    fn requires_surface_read(&self) -> bool {
        true
    }

    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_filter_below(self);
    }

    fn layers(&self) -> &[Arc<dyn Layer>] {
        self.layers.as_slice()
    }

    fn with_layers(&self, layers: Vec<Arc<dyn Layer>>) -> Arc<dyn Layer> {
        Arc::new(Self {
            layers,
            filter: self.filter.clone(),
            geometry: self.geometry.clone(),
            offset: self.offset.clone(),
        })
    }

    fn clone_arc(&self) -> Arc<dyn Layer> {
        Arc::new(self.clone())
    }

    fn any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Filter {
    Blur(BlurFilter),
}

impl Filter {
    pub fn blur(sigma: Radius) -> Self {
        Self::Blur(BlurFilter::new(sigma))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BlurFilter {
    sigma: Radius,
}

impl BlurFilter {
    pub fn new(sigma: Radius) -> Self {
        Self { sigma }
    }

    pub fn sigma(&self) -> &Radius {
        &self.sigma
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OffsetLayer, OpacityLayer};

    fn blur() -> FilterBelowLayer {
        FilterBelowLayer::new(
            Filter::blur(Radius::new(2.0, 2.0)),
            Geometry::None,
            Point::zero(),
        )
    }

    #[test]
    fn surface_reads_are_required_only_for_trees_with_filter_below() {
        let plain = OffsetLayer::new().with_layers(vec![OpacityLayer::new().clone_arc()]);
        assert!(!plain.requires_surface_read());
        assert!(blur().requires_surface_read());
        let nested = plain.with_layers(vec![
            OffsetLayer::new().clone_arc(),
            OpacityLayer::new().with_layers(vec![
                OffsetLayer::new().with_layers(vec![blur().clone_arc()]),
            ]),
        ]);
        assert!(nested.requires_surface_read());
        assert!(nested.clone_arc().requires_surface_read());
        assert!(!nested.with_layers(vec![]).requires_surface_read());
        assert!(
            !plain.requires_surface_read(),
            "other trees remain independent"
        );
    }
}
