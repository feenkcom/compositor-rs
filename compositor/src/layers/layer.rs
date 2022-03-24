use crate::Compositor;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Layer: Debug {
    fn layers(&self) -> &[Arc<dyn Layer>];
    fn count_layers(&self) -> usize {
        self.layers().len()
    }

    fn compose(&self, compositor: &mut dyn Compositor);
}
