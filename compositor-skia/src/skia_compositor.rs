use crate::renderers::PictureToRasterize;
use crate::utils::{clip_canvas, draw_image, draw_shadow};
use crate::{
    as_skia_point, into_skia_matrix, to_skia_point, Cache, PictureRasterizer, ShadowRasterizer,
    ShadowToRasterize,
};
use compositor::{
    ClipLayer, Compositor, Layer, LeftoverStateLayer, OffsetLayer, PictureLayer, Shadow,
    ShadowLayer, StateCommandType, TransformationLayer,
};
use log::error;
use skia_safe::{Canvas, Matrix, Vector};
use std::ops::Neg;
use std::sync::Arc;

#[derive(Debug)]
pub struct SkiaCompositor<'canvas, 'cache> {
    canvas: &'canvas Canvas,
    cache: &'cache mut Cache,
}

impl<'canvas, 'cache> Compositor for SkiaCompositor<'canvas, 'cache> {
    fn compose(&mut self, layer: Arc<dyn Layer>) {
        self.cache.mark_images_as_not_used();

        layer.compose(self);

        self.cache.remove_unused_images();
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

    fn compose_shadow(&mut self, layer: &ShadowLayer) {
        let canvas = &mut self.canvas;

        match self.cache.get_shadow_image(layer.shadow()) {
            None => {
                let rasterized_shadow = ShadowRasterizer::new()
                    .rasterize(ShadowToRasterize::new(layer.shadow().clone()), canvas);

                match rasterized_shadow.image {
                    None => {
                        self.draw_shadow(layer.shadow());
                    }
                    Some(image) => {
                        draw_image(
                            canvas,
                            &image,
                            &Matrix::new_identity(),
                            &layer
                                .shadow()
                                .cull_rect()
                                .translate(&layer.shadow().inflation_offset().neg()),
                        );

                        self.cache.push_shadow_image(layer.shadow().clone(), image);
                    }
                }
            }
            Some(image) => {
                draw_image(
                    canvas,
                    image,
                    &Matrix::new_identity(),
                    &layer
                        .shadow()
                        .cull_rect()
                        .translate(&layer.shadow().inflation_offset().neg()),
                );
            }
        }

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
        let canvas = &mut self.canvas;

        match self.cache.get_picture_image(layer.id()) {
            None => {
                let compositor_picture = layer.picture();
                let picture = compositor_picture
                    .any()
                    .downcast_ref::<skia_safe::Picture>()
                    .expect("Picture is not Skia Picture!");

                if layer.needs_cache() {
                    let rasterized_picture = PictureRasterizer::new().rasterize(
                        PictureToRasterize::new(picture.clone(), canvas.local_to_device_as_3x3()),
                        canvas,
                    );

                    match rasterized_picture.image {
                        None => {
                            error!("Failed to rasterize picture");
                            canvas.draw_picture(picture, None, None);
                        }
                        Some(image) => {
                            draw_image(
                                canvas,
                                &image,
                                &rasterized_picture.matrix,
                                &layer.cull_rect(),
                            );

                            self.cache
                                .push_id_image(layer.id(), image, rasterized_picture.matrix);
                        }
                    }
                } else {
                    canvas.draw_picture(picture, None, None);
                }
            }
            Some((image, matrix)) => {
                draw_image(canvas, &image, &matrix, &layer.cull_rect());
            }
        }
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
}

impl<'canvas, 'cache> SkiaCompositor<'canvas, 'cache> {
    pub fn new(canvas: &'canvas Canvas, cache: &'cache mut Cache) -> Self {
        Self { canvas, cache }
    }

    /// Draws a given shadow directly on the canvas avoiding caches and rasterization
    fn draw_shadow(&mut self, shadow: &Shadow) {
        draw_shadow(self.canvas, shadow, as_skia_point(shadow.offset()).clone());
    }
}
