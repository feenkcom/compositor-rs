use crate::{
    PictureRasterizer, PictureToRasterize, RasterizationStats, RasterizedPicture, RasterizedShadow,
    RasterizerSurfaceType, ShadowRasterizer, ShadowToRasterize,
};
use log::error;
use skia_safe::{surfaces, Canvas, ImageInfo, Surface};

pub trait Rasterizer {
    fn rasterize_picture(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<PictureToRasterize>,
    ) -> Vec<RasterizedPicture>;

    fn rasterize_shadow(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<ShadowToRasterize>,
    ) -> Vec<RasterizedShadow>;
}

pub struct SyncRasterizer {}

impl SyncRasterizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rasterizer for SyncRasterizer {
    fn rasterize_picture(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<PictureToRasterize>,
    ) -> Vec<RasterizedPicture> {
        let mut rasterized_pictures: Vec<RasterizedPicture> = vec![];

        let picture_rasterizer = PictureRasterizer::new();

        for picture in to_rasterize {
            rasterized_pictures.push(picture_rasterizer.rasterize(picture, canvas));
        }
        rasterized_pictures
    }

    fn rasterize_shadow(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<ShadowToRasterize>,
    ) -> Vec<RasterizedShadow> {
        let mut rasterized_shadows: Vec<RasterizedShadow> = vec![];

        let shadow_rasterizer = ShadowRasterizer::new();

        for shadow in to_rasterize {
            rasterized_shadows.push(shadow_rasterizer.rasterize(shadow, canvas));
        }
        rasterized_shadows
    }
}

#[allow(unused_variables)]
pub(crate) fn create_surface(
    canvas: &mut Canvas,
    stats: &mut RasterizationStats,
    image_info: &ImageInfo,
) -> Option<Surface> {
    let surface = {
        #[cfg(feature = "gpu")]
        {
            match canvas.recording_context().as_mut() {
                None => None,
                Some(context) => {
                    let gpu_surface_time = std::time::Instant::now();
                    match Surface::new_render_target(
                        context,
                        skia_safe::Budgeted::Yes,
                        &image_info,
                        0,
                        skia_safe::gpu::SurfaceOrigin::BottomLeft,
                        None,
                        false,
                    ) {
                        Some(surface) => {
                            stats.log(gpu_surface_time, String::from("Create GPU Surface"));
                            stats.set_surface_type(RasterizerSurfaceType::GPU);
                            Some(surface)
                        }
                        None => {
                            error!(
                                "Could not create GPU surface of size {:?}",
                                image_info.dimensions()
                            );
                            None
                        }
                    }
                }
            }
        }
        #[cfg(not(feature = "gpu"))]
        {
            None
        }
    };

    let surface = match surface {
        None => {
            let cpu_surface_time = std::time::Instant::now();
            match surfaces::raster(&image_info, None, None) {
                None => {
                    error!(
                        "Could not create CPU surface of size {:?}",
                        image_info.dimensions()
                    );
                    None
                }
                Some(surface) => {
                    stats.log(cpu_surface_time, String::from("Create CPU Surface"));
                    stats.set_surface_type(RasterizerSurfaceType::Software);
                    Some(surface)
                }
            }
        }
        Some(surface) => Some(surface),
    };
    surface
}
