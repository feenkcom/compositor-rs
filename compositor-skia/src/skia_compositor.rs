use crate::image_cache::ImageCache;
use crate::renderers::PictureToRasterize;
use crate::shadow_cache::ShadowCache;
use crate::{
    as_skia_point, into_skia_matrix, into_skia_rect, into_skia_rrect, to_skia_point,
    PictureRasterizer, ShadowRasterizer, ShadowToRasterize, SkiaPath,
};
use compositor::{
    ClipLayer, Compositor, Geometry, Layer, LeftoverStateLayer, OffsetLayer, PictureLayer,
    Rectangle, Shadow, ShadowLayer, StateCommandType, TransformationLayer,
};
use log::{error, trace};
use skia_safe::image_filters::drop_shadow_only;
use skia_safe::paint::Style;
use skia_safe::{
    scalar, BlendMode, Canvas, ClipOp, Color, Image, Matrix, Paint, PathDirection, Point, Vector,
    M44,
};
use std::ops::Neg;
use std::sync::Arc;

#[derive(Debug)]
pub struct SkiaCompositor<'canvas, 'cache> {
    canvas: &'canvas mut Canvas,
    shadow_cache: &'cache mut ShadowCache,
    image_cache: &'cache mut ImageCache,
}

impl<'canvas, 'cache> Compositor for SkiaCompositor<'canvas, 'cache> {
    fn compose(&mut self, layer: Arc<dyn Layer>) {
        self.image_cache.mark_images_as_not_used();
        self.shadow_cache.mark_images_as_not_used();

        layer.compose(self);

        self.image_cache.remove_unused_images();
        self.shadow_cache.remove_unused_images();
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

        match self.shadow_cache.get_shadow_image(layer.shadow()) {
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

                        self.shadow_cache
                            .push_shadow_image(layer.shadow().clone(), image);
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

        match self.image_cache.get_picture_image(layer.id()) {
            None => {
                let picture = layer
                    .picture()
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

                            self.image_cache.push_id_image(
                                layer.id(),
                                image,
                                rasterized_picture.matrix,
                            );
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
    pub fn new(
        canvas: &'canvas mut Canvas,
        image_cache: &'cache mut ImageCache,
        shadow_cache: &'cache mut ShadowCache,
    ) -> Self {
        Self {
            canvas,
            shadow_cache,
            image_cache,
        }
    }

    /// Draws a given shadow directly on the canvas avoiding caches and rasterization
    fn draw_shadow(&mut self, shadow: &Shadow) {
        draw_shadow(self.canvas, shadow, as_skia_point(shadow.offset()).clone());
    }
}

fn clip_canvas(canvas: &mut Canvas, geometry: &Geometry, offset: Option<&compositor::Point>) {
    match geometry {
        Geometry::Rectangle(rectangle) => {
            let rectangle = offset.map_or(rectangle.clone(), |offset| rectangle.translate(offset));
            canvas.clip_rect(into_skia_rect(&rectangle), ClipOp::Intersect, true);
        }
        Geometry::Path(path) => {
            let skia_path = path
                .any()
                .downcast_ref::<SkiaPath>()
                .expect("Path is not Skia path!")
                .path();

            match offset {
                None => {
                    canvas.clip_path(skia_path, ClipOp::Intersect, true);
                }
                Some(offset) => {
                    canvas.clip_path(
                        &skia_path.with_offset(as_skia_point(offset).clone()),
                        ClipOp::Intersect,
                        true,
                    );
                }
            }
        }
        Geometry::RoundedRectangle(rounded_rectangle) => {
            canvas.clip_rrect(into_skia_rrect(rounded_rectangle), ClipOp::Intersect, true);
        }
        Geometry::None => {}
        Geometry::Circle(circle) => {
            let path = skia_safe::Path::circle(
                as_skia_point(circle.center()).clone(),
                circle.radius().into(),
                PathDirection::CW,
            );
            canvas.clip_path(&path, ClipOp::Intersect, true);
        }
    }
}

/// Draw a given image with the following matrix
fn draw_image(canvas: &mut Canvas, image: &Image, matrix: &Matrix, cull_rectangle: &Rectangle) {
    let current_matrix = canvas.local_to_device_as_3x3();

    let device_bounds = PictureToRasterize::compute_device_bounds_rect(
        &into_skia_rect(cull_rectangle),
        &current_matrix,
    );

    canvas.save();

    let relative_matrix = Matrix::concat(&current_matrix, matrix.invert().as_ref().unwrap());

    let relative_bounds = PictureToRasterize::compute_device_bounds(
        &device_bounds.into(),
        &relative_matrix.invert().unwrap(),
    );

    canvas.reset_matrix();
    canvas.set_matrix(&M44::from(relative_matrix));

    let position = Point::new(relative_bounds.left as f32, relative_bounds.top as f32);
    trace!("Draw image at {:?}", &position);
    canvas.draw_image(image, position, None);
    canvas.restore();
}

pub(crate) fn draw_shadow(canvas: &mut Canvas, shadow: &Shadow, offset: Point) {
    trace!("Draw {:?}", shadow);

    let shadow_offset: Vector = offset + to_skia_point(shadow.inflation_offset());
    let shadow_radius: (scalar, scalar) = shadow.radius().as_tuple_f32();
    let shadow_color: Color = Color::new(shadow.color().as_argb());
    let stroke_width = if shadow_radius.0 > shadow_radius.1 {
        shadow_radius.0
    } else {
        shadow_radius.1
    };

    let drop_shadow_filter =
        drop_shadow_only(shadow_offset, shadow_radius, shadow_color, None, None);

    let mut shadow_paint = Paint::default();
    shadow_paint.set_style(Style::Stroke);
    shadow_paint.set_color(Color::WHITE);
    shadow_paint.set_anti_alias(true);
    shadow_paint.set_blend_mode(BlendMode::SrcOver);
    shadow_paint.set_stroke_width(stroke_width);
    shadow_paint.set_image_filter(drop_shadow_filter);

    draw_geometry(canvas, shadow.geometry(), &shadow_paint);
}

pub(crate) fn draw_geometry(canvas: &mut Canvas, geometry: &Geometry, paint: &Paint) {
    match geometry {
        Geometry::None => {}
        Geometry::Rectangle(rectangle) => {
            canvas.draw_rect(into_skia_rect(rectangle), &paint);
        }
        Geometry::RoundedRectangle(rounded_rectangle) => {
            canvas.draw_rrect(into_skia_rrect(rounded_rectangle), &paint);
        }
        Geometry::Path(path) => {
            let skia_path = path
                .any()
                .downcast_ref::<SkiaPath>()
                .expect("Path is not Skia path!");
            canvas.draw_path(skia_path.path(), &paint);
        }
        Geometry::Circle(circle) => {
            canvas.draw_circle(
                as_skia_point(circle.center()).clone(),
                circle.radius().into(),
                &paint,
            );
        }
    }
}
