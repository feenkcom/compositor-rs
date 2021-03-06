use crate::{
    ClipLayer, Layer, LeftoverStateLayer, OffsetLayer, PictureLayer, ShadowLayer,
    TransformationLayer,
};
use std::fmt::Debug;
use std::sync::Arc;

pub trait Compositor: Debug {
    fn compose(&mut self, layer: Arc<dyn Layer>);
    fn compose_clip(&mut self, layer: &ClipLayer);
    fn compose_offset(&mut self, layer: &OffsetLayer);
    fn compose_shadow(&mut self, layer: &ShadowLayer);
    fn compose_transformation(&mut self, layer: &TransformationLayer);
    fn compose_picture(&mut self, layer: &PictureLayer);
    fn compose_leftover(&mut self, layer: &LeftoverStateLayer);
}
