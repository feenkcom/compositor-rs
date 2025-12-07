use std::ops::Neg;
use std::sync::Arc;

use log::error;

use compositor::{ClipLayer, Compositor, DynamicOffsetLayer, ExplicitLayer, Extent, Layer, LeftoverStateLayer, OffsetLayer, OpacityLayer, Picture, PictureLayer, Point, Shadow, ShadowLayer, StateCommandType, Texture, TextureLayer, TiledLayer, TransformationLayer};
use compositor_skia_platform::Platform;
use skia_safe::gpu::{Budgeted, SurfaceOrigin};
use skia_safe::surface::BackendHandleAccess;
use skia_safe::{
    gpu, AlphaType, Canvas, Color4f, ColorType, Font, Image, ImageInfo, Matrix, Paint,
    PictureRecorder, Point as SkPoint, RRect, Rect, Size, Vector,
};

use crate::renderers::PictureToRasterize;
use crate::textures::disassemble_backend_texture;
use crate::utils::{clip_canvas, draw_image, draw_shadow};
use crate::{
    as_skia_point, into_skia_matrix, to_skia_point, Cache, PictureRasterizer, ShadowRasterizer,
    ShadowToRasterize, SkiaDrawable, SkiaPicture,
};

#[derive(Debug)]
pub struct SkiaCompositor<'canvas, 'cache> {
    platform: Option<Platform>,
    canvas: &'canvas Canvas,
    cache: &'cache mut Cache,
    alpha: Option<f32>,
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

    fn compose_dynamic_offset(&mut self, layer: &DynamicOffsetLayer) {
        let offset = Vector::from(layer.offset().as_tuple_f32());

        self.canvas.save();
        self.canvas.translate(offset);

        for layer in layer.layers() {
            layer.compose(self);
        }

        self.canvas.restore();
    }

    fn compose_opacity(&mut self, layer: &OpacityLayer) {
        let previous_alpha = self.alpha.clone();
        let new_alpha = previous_alpha
            .map(|alpha| alpha * layer.alpha())
            .unwrap_or_else(|| layer.alpha());
        self.alpha = Some(new_alpha);

        for layer in layer.layers() {
            layer.compose(self);
        }

        self.alpha = previous_alpha;
    }

    fn compose_shadow(&mut self, layer: &ShadowLayer) {
        let paint = self.create_layer_paint();
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
                            paint.as_ref(),
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
                    paint.as_ref(),
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
        match self.cache.get_picture_image(layer.id()) {
            None => {
                let canvas = &mut self.canvas;
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
                            canvas.draw_picture(picture, None, self.create_layer_paint().as_ref());
                        }
                        Some(image) => {
                            draw_image(
                                canvas,
                                &image,
                                &rasterized_picture.matrix,
                                &layer.cull_rect(),
                                self.create_layer_paint().as_ref(),
                            );

                            self.cache
                                .push_id_image(layer.id(), image, rasterized_picture.matrix);
                        }
                    }
                } else {
                    canvas.draw_picture(picture, None, self.create_layer_paint().as_ref());
                }
            }
            Some((image, matrix)) => {
                let current_canvas_scale =
                    self.canvas.local_to_device_as_3x3().decompose_scale(None);
                let image_scale = matrix.decompose_scale(None);

                if current_canvas_scale == image_scale {
                    let mut paint = self
                        .create_layer_paint()
                        .unwrap_or_else(|| Paint::default());
                    paint.set_anti_alias(true);

                    draw_image(
                        self.canvas,
                        &image,
                        &matrix,
                        &layer.cull_rect(),
                        Some(&paint),
                    );
                } else {
                    self.cache.remove_picture_image(layer.id());
                    let compositor_picture = layer.picture();
                    let picture = compositor_picture
                        .any()
                        .downcast_ref::<skia_safe::Picture>()
                        .expect("Picture is not Skia Picture!");
                    self.canvas
                        .draw_picture(picture, None, self.create_layer_paint().as_ref());
                }
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

    fn compose_tiled(&mut self, layer: &TiledLayer) {
        let offset = layer.canvas_offset();

        self.canvas.save();
        self.canvas
            .translate(Vector::new(offset.x().into(), offset.y().into()));

        self.scale_tiled_layer(layer);

        layer.visible_tiles().into_iter().for_each(|tile| {
            if layer.is_debug_mode() {
                let rect = Rect::new(
                    tile.left().into(),
                    tile.top().into(),
                    tile.right().into(),
                    tile.bottom().into(),
                );

                let mut paint = Paint::new(Color4f::new(0.8, 0.8, 0.8, 1.0), None);
                paint.set_stroke(true);
                paint.set_stroke_width(0.5);

                self.canvas.draw_rect(&rect, &paint);
            }

            let mut tile_picture = layer.get_tile_picture(&tile);
            if tile_picture.is_none() {
                let mut recorder = PictureRecorder::new();
                let canvas = recorder.begin_recording(
                    Rect::new(0.0, 0.0, tile.width().into(), tile.height().into()),
                    false,
                );

                let mut compositor = SkiaCompositor::new(self.platform.clone(), canvas, self.cache);

                for figure in layer.figures_overlapping_tile(&tile) {
                    if let Some(picture) = figure.get_picture() {
                        let picture_with_offset = OffsetLayer::wrap_with_offset(
                            picture,
                            figure.offset().clone() - tile.origin(),
                        );
                        picture_with_offset.compose(&mut compositor);
                    }
                }

                let recorded_tile_picture: Option<PictureLayer> = recorder
                    .finish_recording_as_picture(None)
                    .map(|tile_picture| {
                        Arc::new(SkiaPicture::new(tile_picture)) as Arc<dyn Picture>
                    })
                    .map(|tile_picture| PictureLayer::new(tile_picture, true));

                if let Some(ref picture) = recorded_tile_picture {
                    layer.cache_tile_picture(&tile, picture.clone());
                };

                tile_picture = recorded_tile_picture;
            }

            let tile_picture = tile_picture.map(|picture| {
                let _image = self.cache.get_picture_image(picture.id());
                OffsetLayer::wrap_with_offset(picture, tile.origin())
            });

            if let Some(tile_picture) = tile_picture {
                tile_picture.compose(self)
            }
        });

        self.canvas.restore();

        if layer.is_debug_mode() {
            self.debug_tiled_layer(layer);
        }
    }

    fn compose_explicit(&mut self, layer: &ExplicitLayer) {
        let drawable = layer
            .drawable()
            .any()
            .downcast_ref::<SkiaDrawable>()
            .expect("Drawable is not Skia Drawable!");

        match drawable {
            SkiaDrawable::Dynamic(rendering) => rendering(self.canvas),
        }
    }

    fn compose_texture(&mut self, layer: &TextureLayer) {
        match layer.texture() {
            Texture::Borrowed(texture) => {
                let mut context = self.canvas.direct_context().unwrap();

                let scale = self
                    .canvas
                    .local_to_device_as_3x3()
                    .decompose_scale(None)
                    .unwrap_or_else(|| Size::new(1.0, 1.0));

                let mut render_target = gpu::surfaces::render_target(
                    &mut context,
                    Budgeted::Yes, // let Skia manage the texture memory
                    &ImageInfo::new_n32_premul(
                        (
                            (layer.width() as f32 * scale.width) as i32,
                            (layer.height() as f32 * scale.height) as i32,
                        ),
                        None,
                    ),
                    Some(1),                // sample count
                    SurfaceOrigin::TopLeft, // texture origin
                    None,                   // optional surface properties
                    false,                  // mipmapped
                    false,
                )
                .unwrap();

                let backend_texture = gpu::surfaces::get_backend_texture(
                    &mut render_target,
                    BackendHandleAccess::FlushRead,
                )
                .unwrap();
                let texture_description = disassemble_backend_texture(
                    self.platform.as_ref(),
                    &mut context,
                    &backend_texture,
                    scale,
                )
                .unwrap();

                context.flush_and_submit();

                (texture.rendering)(texture_description, texture.payload);

                if let Some(image) = Image::from_texture(
                    &mut context,
                    &backend_texture,
                    SurfaceOrigin::TopLeft,
                    ColorType::RGBA8888,
                    AlphaType::Premul,
                    None, // no color space
                ) {
                    // Draw the image into the original canvas
                    let dst =
                        Rect::from_xywh(0.0, 0.0, layer.width() as f32, layer.height() as f32);
                    self.canvas
                        .draw_image_rect(&image, None, dst, &Paint::default());
                }
            }
            Texture::External(_) => {}
        }
    }
}

impl<'canvas, 'cache> SkiaCompositor<'canvas, 'cache> {
    pub fn new(
        platform: Option<Platform>,
        canvas: &'canvas Canvas,
        cache: &'cache mut Cache,
    ) -> Self {
        Self {
            platform,
            canvas,
            cache,
            alpha: None,
        }
    }

    /// Draws a given shadow directly on the canvas avoiding caches and rasterization
    fn draw_shadow(&mut self, shadow: &Shadow) {
        draw_shadow(
            self.canvas,
            shadow,
            as_skia_point(shadow.offset()).clone(),
            self.alpha,
        );
    }

    fn debug_tiled_layer(&mut self, layer: &TiledLayer) {
        let center_x = layer.viewport_width() / 2.0;
        let center_y = layer.viewport_height() / 2.0;
        let rect = RRect::new_oval(Rect::new(
            (center_x - 5.0).into(),
            (center_y - 5.0).into(),
            (center_x + 5.0).into(),
            (center_y + 5.0).into(),
        ));
        let paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 0.5), None);
        self.canvas.draw_rrect(&rect, &paint);

        let extent = Extent::new(280.0, 200.0);
        let origin = Point::new(0.0, 0.0);
        let corner = origin + extent.into();

        let rect = Rect::new(
            origin.x().into(),
            origin.y().into(),
            corner.x().into(),
            corner.y().into(),
        );
        let paint = Paint::new(Color4f::new(1.0, 1.0, 1.0, 0.5), None);
        self.canvas.draw_rect(&rect, &paint);

        let text_paint = Paint::new(Color4f::new(0.1, 0.1, 0.1, 1.0), None);
        let font = Font::default();

        let property_x: f32 = (origin.x() + 20.0).into();
        let property_y: f32 = (origin.y() + 20.0).into();
        let value_x: f32 = (origin.x() + 170.0).into();
        let value_y: f32 = property_y;

        self.canvas.draw_str(
            "Camera position:",
            SkPoint::new(property_x, property_y),
            &font,
            &text_paint,
        );
        self.canvas.draw_str(
            &format!("{}", layer.camera_position()),
            SkPoint::new(value_x, value_y),
            &font,
            &text_paint,
        );

        self.canvas.draw_str(
            "Scale factor:",
            SkPoint::new(property_x, property_y * 2.0),
            &font,
            &text_paint,
        );
        self.canvas.draw_str(
            &format!("{}", layer.scale_factor()),
            SkPoint::new(value_x, value_y * 2.0),
            &font,
            &text_paint,
        );

        self.canvas.draw_str(
            "Zoom level:",
            SkPoint::new(property_x, property_y * 3.0),
            &font,
            &text_paint,
        );
        self.canvas.draw_str(
            &format!("{}", layer.scale_factor().zoom_level()),
            SkPoint::new(value_x, value_y * 3.0),
            &font,
            &text_paint,
        );

        self.canvas.draw_str(
            "Tile extent",
            SkPoint::new(property_x, property_y * 4.0),
            &font,
            &text_paint,
        );
        self.canvas.draw_str(
            &format!("{}", layer.tile_extent()),
            SkPoint::new(value_x, value_y * 4.0),
            &font,
            &text_paint,
        );

        self.canvas.draw_str(
            "Tile scaled extent",
            SkPoint::new(property_x, property_y * 5.0),
            &font,
            &text_paint,
        );
        self.canvas.draw_str(
            &format!("{}", layer.tile_scaled_extent()),
            SkPoint::new(value_x, value_y * 5.0),
            &font,
            &text_paint,
        );

        self.canvas.draw_str(
            "Tiles",
            SkPoint::new(property_x, property_y * 6.0),
            &font,
            &text_paint,
        );
        self.canvas.draw_str(
            &format!(
                "left: {} top: {} right: {} bottom: {}",
                layer.left_tile_column(),
                layer.top_tile_row(),
                layer.right_tile_column(),
                layer.bottom_tile_row()
            ),
            SkPoint::new(value_x, value_y * 6.0),
            &font,
            &text_paint,
        );
    }

    /// Apply scale transformation when composing a tiled layer.
    fn scale_tiled_layer(&mut self, layer: &TiledLayer) {
        let offset = layer.camera_position().clone();
        self.canvas
            .translate(Vector::new(offset.x().into(), offset.y().into()));

        self.canvas
            .scale((layer.scale_factor().value(), layer.scale_factor().value()));

        self.canvas
            .translate(Vector::new(offset.x().into(), offset.y().into()).neg());
    }

    fn create_layer_paint(&mut self) -> Option<Paint> {
        self.alpha.map(|alpha| {
            let mut paint = Paint::default();
            paint.set_alpha_f(alpha);
            paint
        })
    }
}
