#[cfg(feature = "phlow")]
mod extensions {
    use crate::SkiaCachelessCompositor;
    use compositor::{Compositor, Layer, TiledLayer, TiledLayerFigure};
    use phlow::{phlow, phlow_all, PhlowBitmap, PhlowObject, PhlowView};
    use skia_safe::{Color, Color4f, IPoint, ISize, ImageInfo, Paint, Point, Rect, Surface};

    use super::*;

    fn surface_to_bitmap(surface: &mut Surface) -> PhlowBitmap {
        let dst_image_info =
            ImageInfo::new_n32_premul(ISize::new(surface.width(), surface.height()), None);
        let mut pixels: Vec<u8> =
            vec![0; (dst_image_info.width() * dst_image_info.height() * 4) as usize];
        surface.read_pixels(
            &dst_image_info,
            pixels.as_mut_slice(),
            (dst_image_info.width() * 4) as usize,
            IPoint::new(0, 0),
        );

        PhlowBitmap::rgba8(pixels, surface.width(), surface.height())
    }

    #[phlow::extensions(CompositorSkiaExtensions, TiledLayer)]
    impl TiledLayerSkiaExtensions {
        #[phlow::view]
        pub fn preview_for(_this: &TiledLayer, view: impl PhlowView) -> impl PhlowView {
            view.bitmap()
                .title("Preview")
                .priority(4)
                .bitmap::<TiledLayer>(|layer| {
                    let size = ISize::new(
                        Into::<f32>::into(layer.viewport_width()) as i32,
                        Into::<f32>::into(layer.viewport_height()) as i32,
                    );

                    let mut surface = skia_safe::surfaces::raster_n32_premul(size).unwrap();
                    let rect = Rect::new(0.0, 0.0, surface.width() as f32, surface.height() as f32);
                    surface
                        .canvas()
                        .draw_rect(&rect, &Paint::new(Color4f::new(0.8, 0.8, 0.8, 1.0), None));

                    let mut compositor = SkiaCachelessCompositor::new(surface.canvas());
                    compositor.compose(layer.clone_arc());

                    surface_to_bitmap(&mut surface)
                })
        }
    }
}
