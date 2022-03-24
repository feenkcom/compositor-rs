use crate::Compositor;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Layer: Debug {
    fn compose(&self, compositor: &mut dyn Compositor);
    fn layers(&self) -> &[Arc<dyn Layer>];
    fn count_layers(&self) -> usize {
        self.layers().len()
    }
    fn clone_arc(&self) -> Arc<dyn Layer>;
    fn any(&self) -> &dyn Any;
}
