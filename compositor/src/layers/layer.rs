use crate::Compositor;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Layer: Send + Sync + Debug {
    fn compose(&self, compositor: &mut dyn Compositor);
    fn layers(&self) -> &[Arc<dyn Layer>];
    /// Whether this subtree needs to read pixels already drawn on the surface.
    fn requires_surface_read(&self) -> bool {
        self.layers()
            .iter()
            .any(|layer| layer.requires_surface_read())
    }
    /// Create a copy of the layer with the vector of layers as its children.
    /// The resulting layer does not preserve existing children layers
    fn with_layers(&self, layers: Vec<Arc<dyn Layer>>) -> Arc<dyn Layer>;
    fn count_layers(&self) -> usize {
        self.layers().len()
    }
    fn clone_arc(&self) -> Arc<dyn Layer>;
    fn any(&self) -> &dyn Any;
}
