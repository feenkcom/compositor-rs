use crate::utils::{clip_canvas, draw_shadow};
use crate::{as_skia_point, into_skia_matrix, to_skia_point};
use compositor::{
    ClipLayer, Compositor, Layer, LeftoverStateLayer, OffsetLayer, OpacityLayer, PictureLayer,
    Shadow, ShadowLayer, StateCommandType, TiledLayer, TransformationLayer,
};
use skia_safe::{Canvas, Color4f, Paint, Rect, Vector};
use std::sync::Arc;

#[derive(Debug)]
pub struct SkiaCachelessCompositor<'canvas> {
    canvas: &'canvas Canvas,
}

impl<'canvas> Compositor for SkiaCachelessCompositor<'canvas> {
    fn compose(&mut self, layer: Arc<dyn Layer>) {
        layer.compose(self);
    }

    fn compose_clip(&mut self, layer: &ClipLayer) {
        let count = self.canvas.save();

        for layer in layer.layers() {
            layer.compose(self);
        }

        self.canvas.restore_to_count(count);
    }

    fn compose_offset(&mut self, layer: &OffsetLayer) {
        let offset = Vector::from(layer.offset().as_tuple_f32());

        self.canvas.save();
        self.canvas.translate(offset);

        for layer in layer.layers() {
            layer.compose(self);
        }

        self.canvas.restore();
    }

    fn compose_opacity(&mut self, layer: &OpacityLayer) {
        todo!()
    }

    fn compose_shadow(&mut self, layer: &ShadowLayer) {
        self.draw_shadow(layer.shadow());

        for layer in layer.layers() {
            layer.compose(self);
        }
    }

    fn compose_transformation(&mut self, layer: &TransformationLayer) {
        let matrix = into_skia_matrix(layer.matrix());

        self.canvas.save();
        self.canvas.concat(&matrix);
        for layer in layer.layers() {
            layer.compose(self);
        }
        self.canvas.restore();
    }

    fn compose_picture(&mut self, layer: &PictureLayer) {
        let compositor_picture = layer.picture();
        let picture = compositor_picture
            .any()
            .downcast_ref::<skia_safe::Picture>()
            .expect("Picture is not Skia Picture!");

        self.canvas.draw_picture(picture, None, None);
    }

    fn compose_leftover(&mut self, layer: &LeftoverStateLayer) {
        let canvas = &mut self.canvas;

        let count = canvas.save();

        for command in &layer.commands {
            match &command.command_type {
                StateCommandType::Transform(matrix) => {
                    canvas.translate(to_skia_point(command.offset.clone()));
                    canvas.concat(&into_skia_matrix(matrix));
                }
                StateCommandType::Clip(clip) => {
                    clip_canvas(canvas, clip, Some(&command.offset));
                }
            }
        }

        for layer in layer.layers() {
            layer.compose(self);
        }

        self.canvas.restore_to_count(count);
    }

    fn compose_tiled(&mut self, layer: &TiledLayer) {
        let offset = layer.canvas_offset();

        self.canvas.save();
        self.canvas
            .translate(Vector::new(offset.x().into(), offset.y().into()));

        for figure in layer.visible_figures() {
            if let Some(picture) = figure.picture() {
                self.compose(Arc::new(picture));
            }
        }

        self.canvas.restore();
    }
}

impl<'canvas> SkiaCachelessCompositor<'canvas> {
    pub fn new(canvas: &'canvas Canvas) -> Self {
        Self { canvas }
    }

    /// Draws a given shadow directly on the canvas avoiding caches and rasterization
    fn draw_shadow(&mut self, shadow: &Shadow) {
        draw_shadow(
            self.canvas,
            shadow,
            as_skia_point(shadow.offset()).clone(),
            None,
        );
    }
}
