use crate::RasterizationStats;
use skia_safe::{
    Canvas, Color, ColorSpace, IRect, Image, ImageInfo, Matrix, Picture, Rect, RoundOut, Vector,
};
use std::fmt::{Debug, Error, Formatter};
use std::sync::Arc;

use crate::renderers::rasterizer::create_surface;

/// I contain all the necessary data to rasterize a picture
#[derive(Clone)]
pub struct PictureToRasterize {
    pub picture: Arc<Picture>,
    pub bounds: Rect,
    pub matrix: Matrix,
}

impl PictureToRasterize {
    pub fn new(picture: Arc<Picture>, matrix: Matrix) -> Self {
        let logical_bounds = picture.cull_rect();
        Self {
            picture,
            bounds: logical_bounds,
            matrix,
        }
    }

    pub fn device_bounds(&self) -> IRect {
        Self::compute_device_bounds(&self.bounds, &self.matrix)
    }

    pub fn into_rasterized(
        self,
        image: Option<Image>,
        stats: RasterizationStats,
    ) -> RasterizedPicture {
        RasterizedPicture::new(self.picture, image, self.matrix, stats)
    }

    pub fn compute_device_bounds(bounds: &Rect, matrix: &Matrix) -> IRect {
        matrix.map_rect(bounds).0.round_out()
    }

    pub fn compute_device_bounds_rect(bounds: &Rect, matrix: &Matrix) -> Rect {
        matrix.map_rect(bounds).0
    }
}

impl Debug for PictureToRasterize {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("PictureToRasterize")
            .field("id", &self.picture.unique_id())
            .field("bounds", &self.bounds)
            .finish()
    }
}

/// I hold a result of the picture rasterization. The image is [`Some`] if the process
/// was successful
pub struct RasterizedPicture {
    pub picture: Arc<Picture>,
    pub image: Option<Image>,
    pub matrix: Matrix,
    pub stats: RasterizationStats,
}

impl RasterizedPicture {
    pub fn new(
        picture: Arc<Picture>,
        image: Option<Image>,
        matrix: Matrix,
        stats: RasterizationStats,
    ) -> Self {
        Self {
            picture,
            image,
            matrix,
            stats,
        }
    }

    pub fn picture_id(&self) -> u32 {
        self.picture.unique_id()
    }
}

/// I convert a Picture [`PictureToRasterize`] into an Image
pub struct PictureRasterizer {}
impl PictureRasterizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rasterize(
        &self,
        picture_to_rasterize: PictureToRasterize,
        canvas: &mut Canvas,
    ) -> RasterizedPicture {
        let device_bounds = picture_to_rasterize.device_bounds();
        let picture = &picture_to_rasterize.picture;
        let picture_id = picture.unique_id();

        let mut stats = RasterizationStats::new(picture_id);
        let start_time = std::time::Instant::now();

        let image_info = ImageInfo::new_n32_premul(device_bounds.size(), ColorSpace::new_srgb());

        let surface = create_surface(canvas, &mut stats, &image_info);

        let image = match surface {
            None => None,
            Some(mut surface) => {
                let draw_picture_time = std::time::Instant::now();

                let canvas = surface.canvas();
                canvas.clear(Color::TRANSPARENT);
                canvas.translate(Vector::new(
                    -device_bounds.left as f32,
                    -device_bounds.top as f32,
                ));
                canvas.concat(&picture_to_rasterize.matrix);

                canvas.draw_picture(&picture, None, None);

                stats.log(draw_picture_time, String::from("Draw picture"));

                let canvas_flush = std::time::Instant::now();
                stats.log(canvas_flush, String::from("Flush canvas"));

                let raster_image_snapshot = std::time::Instant::now();
                let image = Some(surface.image_snapshot());
                stats.log(raster_image_snapshot, String::from("Image snapshot"));
                image
            }
        };

        stats.log_total(start_time);

        picture_to_rasterize.into_rasterized(image, stats)
    }
}
