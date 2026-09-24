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
