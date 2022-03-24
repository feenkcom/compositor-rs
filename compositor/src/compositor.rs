use crate::{
    ClipLayer, LeftoverStateLayer, OffsetLayer, PictureLayer, ShadowLayer, TransformationLayer,
};
use std::fmt::Debug;

pub trait Compositor: Debug {
    fn compose_clip(&mut self, layer: &ClipLayer);
    fn compose_offset(&mut self, layer: &OffsetLayer);
    fn compose_shadow(&mut self, layer: &ShadowLayer);
    fn compose_transformation(&mut self, layer: &TransformationLayer);
    fn compose_picture(&mut self, layer: &PictureLayer);
    fn compose_leftover(&mut self, layer: &LeftoverStateLayer);
}
