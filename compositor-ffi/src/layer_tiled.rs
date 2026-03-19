use std::sync::Arc;

use array_box::ArrayBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

use compositor::{
    Extent, Layer, Picture, PictureLayer, Point, TiledFigureId, TiledLayer, TiledLayerFigure,
    TiledLayerScaleFactor,
};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_default() -> OwnedPtr<Arc<dyn Layer>> {
    OwnedPtr::new(Arc::new(TiledLayer::default()) as Arc<dyn Layer>)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_new(
    camera_x: f32,
    camera_y: f32,
    width: f32,
    height: f32,
    tile_width: f32,
    tile_height: f32,
) -> OwnedPtr<Arc<dyn Layer>> {
    OwnedPtr::new(Arc::new(TiledLayer::new(
        Point::new(camera_x, camera_y),
        Extent::new(width, height),
        Extent::new(tile_width, tile_height)
    )) as Arc<dyn Layer>)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_add_figure(
    layer: BorrowedPtr<Arc<dyn Layer>>,
    id: TiledFigureId,
    offset_x: f32,
    offset_y: f32,
    width: f32,
    height: f32,
) {
    layer
        .with_ref_ok(|layer| {
            let tiled_layer = layer
                .any()
                .downcast_ref::<TiledLayer>()
                .expect("Is not a tiled layer!");

            tiled_layer.add_figure(TiledLayerFigure::new(
                id,
                Point::new_f32(offset_x, offset_y),
                Extent::new(width, height),
            ));
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_figure_set_picture(
    tiled_layer: BorrowedPtr<Arc<dyn Layer>>,
    id: TiledFigureId,
    picture: BorrowedPtr<Arc<dyn Picture>>,
) {
    tiled_layer
        .with_ref_ok(|tiled_layer| {
            picture.with_clone_ok(|picture| {
                let tiled_layer = tiled_layer
                    .any()
                    .downcast_ref::<TiledLayer>()
                    .expect("Is not a tiled layer!");

                let picture_layer = PictureLayer::new(picture, false);

                if let Some(figure) = tiled_layer.find_figure_by_id(id) {
                    figure.set_picture(picture_layer);
                }
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_figure_set_picture_layer(
    tiled_layer: BorrowedPtr<Arc<dyn Layer>>,
    id: TiledFigureId,
    picture_layer: BorrowedPtr<Arc<dyn Layer>>,
) {
    tiled_layer
        .with_ref_ok(|tiled_layer| {
            picture_layer.with_ref_ok(|picture_layer| {
                let tiled_layer = tiled_layer
                    .any()
                    .downcast_ref::<TiledLayer>()
                    .expect("Is not a tiled layer!");

                let picture_layer = picture_layer
                    .any()
                    .downcast_ref::<PictureLayer>()
                    .expect("Is not a picture layer!");

                if let Some(figure) = tiled_layer.find_figure_by_id(id) {
                    figure.set_picture(picture_layer.clone());
                }
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_set_camera_position(
    mut tiled_layer: BorrowedPtr<Arc<dyn Layer>>,
    camera_x: f32,
    camera_y: f32,
) {
    tiled_layer
        .with_mut_ok(|layer| {
            let updated = {
                let tiled_layer = layer
                    .any()
                    .downcast_ref::<TiledLayer>()
                    .expect("Is not a tiled layer!");

                tiled_layer
                    .with_camera_position(Point::new(camera_x, camera_y))
                    .clone_arc()
            };
            *layer = updated;
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_scale_value(
    tiled_layer: BorrowedPtr<Arc<dyn Layer>>,
) -> f32 {
    tiled_layer
        .with_ref_ok(|tiled_layer| {
            let tiled_layer = tiled_layer
                .any()
                .downcast_ref::<TiledLayer>()
                .expect("Is not a tiled layer!");

            tiled_layer.scale_factor().value()
        })
        .or_log(1.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_set_scale_in_factor(
    mut tiled_layer: BorrowedPtr<Arc<dyn Layer>>,
    scale: f32,
) {
    tiled_layer
        .with_mut_ok(|layer| {
            let updated = {
                let tiled_layer = layer
                    .any()
                    .downcast_ref::<TiledLayer>()
                    .expect("Is not a tiled layer!");

                tiled_layer
                    .with_scale_factor(TiledLayerScaleFactor::scale_in(scale))
                    .clone_arc()
            };
            *layer = updated;
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_set_scale_out_factor(
    mut tiled_layer: BorrowedPtr<Arc<dyn Layer>>,
    scale: f32,
) {
    tiled_layer
        .with_mut_ok(|layer| {
            let updated = {
                let tiled_layer = layer
                    .any()
                    .downcast_ref::<TiledLayer>()
                    .expect("Is not a tiled layer!");

                tiled_layer
                    .with_scale_factor(TiledLayerScaleFactor::scale_out(scale))
                    .clone_arc()
            };
            *layer = updated;
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_visible_figures(
    tiled_layer: BorrowedPtr<Arc<dyn Layer>>,
    mut ids: BorrowedPtr<ArrayBox<u32>>,
) {
    tiled_layer
        .with_ref_ok(|tiled_layer| {
            ids.with_mut_ok(|ids| {
                let tiled_layer = tiled_layer
                    .any()
                    .downcast_ref::<TiledLayer>()
                    .expect("Is not a tiled layer!");

                ids.set_vector(
                    tiled_layer
                        .visible_figures()
                        .iter()
                        .map(|each| each.id())
                        .collect(),
                );
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_visible_figures_without_pictures(
    tiled_layer: BorrowedPtr<Arc<dyn Layer>>,
    mut ids: BorrowedPtr<ArrayBox<u32>>,
) {
    tiled_layer
        .with_ref_ok(|tiled_layer| {
            ids.with_mut_ok(|ids| {
                let tiled_layer = tiled_layer
                    .any()
                    .downcast_ref::<TiledLayer>()
                    .expect("Is not a tiled layer!");

                ids.set_vector(tiled_layer.visible_figures_ids_without_picture());
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_tiled_layer_visible_figures_within_tiles_without_pictures(
    tiled_layer: BorrowedPtr<Arc<dyn Layer>>,
    mut ids: BorrowedPtr<ArrayBox<u32>>,
) {
    tiled_layer
        .with_ref_ok(|tiled_layer| {
            ids.with_mut_ok(|ids| {
                let tiled_layer = tiled_layer
                    .any()
                    .downcast_ref::<TiledLayer>()
                    .expect("Is not a tiled layer!");

                ids.set_vector(tiled_layer.visible_figures_ids_within_tiles_without_picture());
            })
        })
        .log();
}
