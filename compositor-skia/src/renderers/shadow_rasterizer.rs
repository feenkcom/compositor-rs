use crate::{as_skia_point, into_skia_rect, RasterizationStats};
use compositor::Shadow;
use log::{error, trace};
use skia_safe::{
    Canvas, Color, ColorSpace, IRect, Image, ImageInfo, Matrix, Point, Rect, RoundOut, Vector,
};

use crate::renderers::rasterizer::create_surface;
use crate::utils::draw_shadow;

#[derive(Debug, Clone)]
pub struct ShadowToRasterize {
    pub shadow: Shadow,
    pub bounds: Rect,
}

impl ShadowToRasterize {
    pub fn new(shadow: Shadow) -> Self {
        let bounds = into_skia_rect(&shadow.cull_rect());
        Self { shadow, bounds }
    }

    pub fn into_rasterized(
        self,
        image: Option<Image>,
        stats: RasterizationStats,
    ) -> RasterizedShadow {
        RasterizedShadow::new(self.shadow, image, stats)
    }

    pub fn compute_device_bounds(bounds: &Rect, matrix: &Matrix) -> IRect {
        matrix.map_rect(bounds).0.round_out()
    }
}

/// I hold a result of the shadow rasterization. The image is [`Some`] if the process
/// was successful
pub struct RasterizedShadow {
    pub shadow: Shadow,
    pub image: Option<Image>,
    pub stats: RasterizationStats,
}

impl RasterizedShadow {
    pub fn new(shadow: Shadow, image: Option<Image>, stats: RasterizationStats) -> Self {
        Self {
            shadow,
            image,
            stats,
        }
    }
}

/// I convert a Shadow [`ShadowToRasterize`] into an Image
pub struct ShadowRasterizer {}
impl ShadowRasterizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rasterize(
        &self,
        shadow_to_rasterize: ShadowToRasterize,
        canvas: &Canvas,
    ) -> RasterizedShadow {
        trace!(
            "Rasterize shadow with bounds = {:?}, cull rectangle = {:?}",
            &shadow_to_rasterize.bounds,
            shadow_to_rasterize.shadow.cull_rect()
        );

        let device_bounds = shadow_to_rasterize.bounds.round();
        trace!(
            "About to rasterize shadow with device bounds {:?}",
            &device_bounds
        );
        let shadow = &shadow_to_rasterize.shadow;

        let mut stats = RasterizationStats::new(0);
        let start_time = std::time::Instant::now();

        trace!(
            "About to create image info of size {:?}",
            &device_bounds.size()
        );
        let image_info = ImageInfo::new_n32_premul(device_bounds.size(), ColorSpace::new_srgb());
        trace!("About to create surface with {:?}", &image_info);

        let surface = create_surface(canvas, &mut stats, &image_info);

        let image = match surface {
            None => {
                error!("Could not rasterize shadow {:?}", shadow);
                None
            }
            Some(mut surface) => {
                let draw_shadow_time = std::time::Instant::now();

                let canvas = surface.canvas();

                canvas.clear(Color::TRANSPARENT);
                canvas
                    .translate(Vector::new(
                        -device_bounds.left as f32,
                        -device_bounds.top as f32,
                    ));
                canvas.translate(as_skia_point(shadow.offset()).clone());

                draw_shadow(canvas, shadow, Point::new(0.0, 0.0), None);

                stats.log(draw_shadow_time, String::from("Draw shadow"));

                let canvas_flush = std::time::Instant::now();
                stats.log(canvas_flush, String::from("Flush canvas"));

                let raster_image_snapshot = std::time::Instant::now();
                let snapshot = surface.image_snapshot();
                trace!("Created shadow snapshot with {:?}", snapshot.bounds());
                let image = Some(snapshot);
                stats.log(raster_image_snapshot, String::from("Image snapshot"));
                image
            }
        };

        stats.log_total(start_time);

        shadow_to_rasterize.into_rasterized(image, stats)
    }
}
