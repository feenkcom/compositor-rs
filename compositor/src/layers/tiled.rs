use fraction::{Fraction, ToPrimitive};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::RwLock;
use rstar::{ParentNode, RTree, RTreeObject, AABB};

use crate::{Compositor, Extent, Layer, OffsetLayer, PictureLayer, Point, Scalar};

pub type RowIndex = i32;
pub type ColumnIndex = i32;
pub type TiledFigureId = u32;

#[derive(Debug, Clone)]
pub struct TiledLayer {
    /// optimized for finding figures overlapping a tile
    figures: Arc<RwLock<RTree<TiledLayerFigure>>>,
    /// optimized for finding figures based on their id
    figures_hash: Arc<RwLock<HashMap<TiledFigureId, TiledLayerFigure>>>,
    camera_position: Point,
    viewport_extent: Extent,
    tile_extent: Extent,
    tile_pictures: Arc<RwLock<HashMap<(ColumnIndex, RowIndex), PictureLayer>>>,
    scale_factor: TiledLayerScaleFactor,
    debug_mode: bool,
}

impl Default for TiledLayer {
    fn default() -> Self {
        Self::new(
            Point::zero(),
            Extent::new(600.0, 400.0),
            Extent::new(128.0, 128.0),
        )
    }
}

impl TiledLayer {
    pub fn new(camera_position: Point, viewport_extent: Extent, tile_extent: Extent) -> Self {
        Self {
            figures: Default::default(),
            figures_hash: Arc::new(Default::default()),
            camera_position,
            viewport_extent,
            tile_extent,
            tile_pictures: Arc::new(Default::default()),
            scale_factor: TiledLayerScaleFactor::scale_in(1.0),
            debug_mode: true,
        }
    }

    pub fn with_camera_position(&self, camera_position: Point) -> Self {
        let mut layer = self.clone();
        layer.camera_position = camera_position;
        layer
    }

    pub fn with_scale_factor(&self, scale_factor: TiledLayerScaleFactor) -> Self {
        let mut layer = self.clone();
        layer.scale_factor = scale_factor;
        layer
    }

    pub fn add_figure(&self, figure: TiledLayerFigure) {
        self.figures.write().insert(figure.clone());
        self.figures_hash.write().insert(figure.id(), figure);
    }

    pub fn camera_position(&self) -> &Point {
        &self.camera_position
    }

    pub fn viewport_extent(&self) -> &Extent {
        &self.viewport_extent
    }

    pub fn tile_extent(&self) -> &Extent {
        &self.tile_extent
    }

    pub fn tile_width(&self) -> Scalar {
        self.tile_extent.width()
    }

    pub fn tile_height(&self) -> Scalar {
        self.tile_extent.height()
    }

    pub fn scale_factor(&self) -> &TiledLayerScaleFactor {
        &self.scale_factor
    }

    pub fn clone_figures(&self) -> Vec<TiledLayerFigure> {
        self.figures
            .read()
            .iter()
            .map(|each| each.clone())
            .collect()
    }

    pub fn clone_tree_root(&self) -> ParentNode<TiledLayerFigure> {
        self.figures.read().root().clone()
    }

    /// Left coordinate of the viewport which depends on the camera position and viewport size
    pub fn viewport_left(&self) -> Scalar {
        self.camera_position.x() - (self.viewport_extent.width() / 2.0)
    }

    /// Top coordinate of the viewport which depends on the camera position and viewport size
    pub fn viewport_top(&self) -> Scalar {
        self.camera_position.y() - (self.viewport_extent.height() / 2.0)
    }

    pub fn viewport_width(&self) -> Scalar {
        self.viewport_extent.width()
    }

    /// Right coordinate of the viewport which depends on the camera position and viewport size
    pub fn viewport_right(&self) -> Scalar {
        self.camera_position.x() + (self.viewport_extent.width() / 2.0)
    }

    /// Bottom coordinate of the viewport which depends on the camera position and viewport size
    pub fn viewport_bottom(&self) -> Scalar {
        self.camera_position.y() + (self.viewport_extent.height() / 2.0)
    }

    pub fn viewport_height(&self) -> Scalar {
        self.viewport_extent.height()
    }

    pub fn canvas_offset(&self) -> Point {
        Into::<Point>::into(self.viewport_extent / 2.0) - self.camera_position
    }

    pub fn tile_scaled_extent(&self) -> Extent {
        Extent::new(self.tile_scaled_width(), self.tile_scaled_height())
    }

    /// Calculate the scale factor of a tile, which is how much a tile at the current zoom level
    /// should be scaled up or down in order to match the viewpoint's scale
    pub fn tile_scale_factor(&self) -> f32 {
        self.scale_factor.tile_scale_factor()
    }

    /// Calculate the width of a tile within the viewport, in viewport coordinates
    pub fn tile_scaled_width(&self) -> Scalar {
        self.tile_extent.width() * self.tile_scale_factor()
    }

    /// Calculate the height of a tile within the viewport, in viewport coordinates
    pub fn tile_scaled_height(&self) -> Scalar {
        self.tile_extent.height() * self.tile_scale_factor()
    }

    pub fn left_tile_column(&self) -> ColumnIndex {
        column_at(self.viewport_left(), self.tile_scaled_width())
    }

    pub fn right_tile_column(&self) -> ColumnIndex {
        column_at(self.viewport_right(), self.tile_scaled_width())
    }

    pub fn top_tile_row(&self) -> RowIndex {
        row_at(self.viewport_top(), self.tile_scaled_height())
    }

    pub fn bottom_tile_row(&self) -> RowIndex {
        row_at(self.viewport_bottom(), self.tile_scaled_height())
    }

    /// Find and return figures that overlap a given tile
    pub fn figures_overlapping_tile(&self, tile: &TiledLayerTile) -> Vec<TiledLayerFigure> {
        self.figures
            .read()
            .locate_in_envelope_intersecting(&tile.envelope())
            .map(|figure| figure.clone())
            .collect()
    }

    /// Return an iterator over visible tiles. It takes scale factor into account when determining which tiles are visible
    pub fn visible_tiles(&self) -> TiledLayerVisibleTilesIterator {
        TiledLayerVisibleTilesIterator::for_tiled_layer(self)
    }

    pub fn figures(&self) -> Vec<TiledLayerFigure> {
        self.figures
            .read()
            .iter()
            .map(|figure| figure.clone())
            .collect()
    }

    /// Find and return all visible figures
    pub fn visible_figures(&self) -> Vec<TiledLayerFigure> {
        self.figures
            .read()
            .locate_in_envelope_intersecting(&self.envelope())
            .map(|each_figure| each_figure.clone())
            .collect()
    }

    /// Find and return all visible figures that don't have a picture
    pub fn visible_figures_without_picture(&self) -> Vec<TiledLayerFigure> {
        self.figures
            .read()
            .locate_in_envelope_intersecting(&self.envelope())
            .filter(|figure| !figure.has_picture())
            .map(|each_figure| each_figure.clone())
            .collect()
    }

    /// Find and return IDs of all visible figures that don't have a picture
    pub fn visible_figures_ids_without_picture(&self) -> Vec<TiledFigureId> {
        self.figures
            .read()
            .locate_in_envelope_intersecting(&self.envelope())
            .filter(|figure| !figure.has_picture())
            .map(|each_figure| each_figure.id())
            .collect()
    }

    /// Find and return IDs of all visible figures that don't have a picture
    pub fn visible_figures_ids_within_tiles_without_picture(&self) -> Vec<TiledFigureId> {
        let mut ids = self
            .visible_tiles()
            .flat_map(|tile| {
                self.figures
                    .read()
                    .locate_in_envelope_intersecting(&tile.envelope())
                    .filter(|figure| !figure.has_picture())
                    .map(|each_figure| each_figure.id())
                    .collect::<Vec<TiledFigureId>>()
                    .into_iter()
            })
            .collect::<Vec<TiledFigureId>>();

        ids.dedup();
        ids
    }

    pub fn find_figure_by_id(&self, id: TiledFigureId) -> Option<TiledLayerFigure> {
        self.figures_hash.read().get(&id).cloned()
    }

    pub fn cache_tile_picture(&self, tile: &TiledLayerTile, picture: PictureLayer) {
        let _ = self
            .tile_pictures
            .write()
            .insert(tile.coordinate(), picture);
    }

    pub fn get_tile_picture(&self, tile: &TiledLayerTile) -> Option<PictureLayer> {
        self.tile_pictures.read().get(&tile.coordinate()).cloned()
    }

    pub fn is_debug_mode(&self) -> bool {
        self.debug_mode
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TiledLayerScaleFactor {
    ScaleIn(f32),
    ScaleOut(f32),
}

impl TiledLayerScaleFactor {
    pub fn scale_in(scale: f32) -> Self {
        Self::ScaleIn(scale)
    }

    pub fn scale_out(scale: f32) -> Self {
        Self::ScaleOut(scale)
    }

    /// Return a zoom level represented as a fraction with integer denominator.
    ///	For example, zoom level of (1 / 4) means that we the scene is zoomed out 4 times.
    ///	Contrarily, zoom level of (4 / 1) means that the scene is zoomed in 4 times.
    pub fn zoom_level(&self) -> Fraction {
        match *self {
            TiledLayerScaleFactor::ScaleIn(scale) => {
                if scale >= 1.0f32 {
                    Fraction::new(scale.floor() as u64, 1u64)
                } else {
                    Fraction::new(1u64, (1.0 / scale).ceil() as u64)
                }
            }
            TiledLayerScaleFactor::ScaleOut(scale) => {
                if scale >= 1.0f32 {
                    Fraction::new(scale.ceil() as u64, 1u64)
                } else {
                    Fraction::new(1u64, (1.0 / scale).floor() as u64)
                }
            }
        }
    }

    pub fn value(&self) -> f32 {
        match self {
            TiledLayerScaleFactor::ScaleIn(scale) => *scale,
            TiledLayerScaleFactor::ScaleOut(scale) => *scale,
        }
    }

    pub fn tile_scale_factor(&self) -> f32 {
        self.value() / self.zoom_level().to_f32().unwrap()
    }
}

impl Display for TiledLayerScaleFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TiledLayerScaleFactor::ScaleIn(scale) => {
                write!(f, "Scale-in {}", scale)
            }
            TiledLayerScaleFactor::ScaleOut(scale) => {
                write!(f, "Scale-out {}", scale)
            }
        }
    }
}

impl RTreeObject for TiledLayer {
    type Envelope = AABB<Point>;

    fn envelope(&self) -> Self::Envelope {
        let corner_1 = Point::new(self.viewport_left(), self.viewport_top());
        let corner_2 = Point::new(self.viewport_right(), self.viewport_bottom());
        AABB::from_corners(corner_1, corner_2)
    }
}

fn column_at(x: Scalar, tile_width: Scalar) -> ColumnIndex {
    let index = x / tile_width;
    (index.trunc() + index.signum()) as ColumnIndex
}

fn row_at(y: Scalar, tile_height: Scalar) -> RowIndex {
    let index = y / tile_height;
    (index.trunc() + index.signum()) as RowIndex
}

#[derive(Debug, Clone)]
pub struct TiledLayerVisibleTilesIterator<'layer> {
    layer: &'layer TiledLayer,
    start_column: ColumnIndex,
    end_column: ColumnIndex,
    start_row: RowIndex,
    end_row: RowIndex,
    current_column: ColumnIndex,
    current_row: RowIndex,
    current_tile: Rc<TiledLayerTile>,
}

impl<'layer> TiledLayerVisibleTilesIterator<'layer> {
    pub fn for_tiled_layer(layer: &'layer TiledLayer) -> Self {
        let start_column = layer.left_tile_column();
        let start_row = layer.top_tile_row();

        let mut tile = TiledLayerTile::default();
        tile.extent = layer.tile_extent.clone();

        Self {
            layer,
            start_column,
            end_column: layer.right_tile_column(),
            start_row,
            end_row: layer.bottom_tile_row(),
            current_column: start_column.clone(),
            current_row: start_row.clone(),
            current_tile: Rc::new(tile),
        }
    }
}

impl<'layer> Iterator for TiledLayerVisibleTilesIterator<'layer> {
    type Item = Rc<TiledLayerTile>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row > self.end_row {
            return None;
        }

        let tile_mut = Rc::make_mut(&mut self.current_tile);
        tile_mut.column = self.current_column;
        tile_mut.row = self.current_row;

        if self.current_column >= self.end_column {
            self.current_column = self.start_column;
            self.current_row = self.current_row + 1;
            if self.current_row == 0 {
                self.current_row = 1;
            }
        } else {
            self.current_column = self.current_column + 1;
            if self.current_column == 0 {
                self.current_column = 1;
            }
        }

        Some(self.current_tile.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TiledLayerVisibleTilesAndTheirFiguresIterator<'layer> {
    layer: &'layer TiledLayer,
    visible_tiles: TiledLayerVisibleTilesIterator<'layer>,
}

impl<'layer> Iterator for TiledLayerVisibleTilesAndTheirFiguresIterator<'layer> {
    type Item = (Rc<TiledLayerTile>, Vec<TiledLayerFigure>);

    fn next(&mut self) -> Option<Self::Item> {
        self.visible_tiles.next().map(|tile| {
            (
                tile.clone(),
                self.layer.figures_overlapping_tile(tile.as_ref()),
            )
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct TiledLayerTile {
    column: ColumnIndex,
    row: RowIndex,
    extent: Extent,
}

impl TiledLayerTile {
    pub fn new(row: impl Into<RowIndex>, column: impl Into<ColumnIndex>) -> Self {
        Self {
            column: column.into(),
            row: row.into(),
            extent: Default::default(),
        }
    }

    pub fn coordinate(&self) -> (ColumnIndex, RowIndex) {
        (self.column, self.row)
    }

    pub fn left(&self) -> Scalar {
        let left = if self.column < 0 {
            self.column
        } else {
            self.column - 1
        };

        self.extent.width() * left as f32
    }

    pub fn top(&self) -> Scalar {
        let top = if self.row < 0 { self.row } else { self.row - 1 };
        self.extent.height() * top as f32
    }

    pub fn right(&self) -> Scalar {
        self.left() + self.width()
    }

    pub fn bottom(&self) -> Scalar {
        self.top() + self.height()
    }

    pub fn origin(&self) -> Point {
        Point::new(self.left(), self.top())
    }

    pub fn width(&self) -> Scalar {
        self.extent.width()
    }

    pub fn height(&self) -> Scalar {
        self.extent.height()
    }
}

impl RTreeObject for TiledLayerTile {
    type Envelope = AABB<Point>;

    fn envelope(&self) -> Self::Envelope {
        let corner_1 = self.origin();
        let corner_2 = Point::new(
            corner_1.x() + self.extent.width(),
            corner_1.y() + self.extent.height(),
        );
        AABB::from_corners(corner_1, corner_2)
    }
}

#[derive(Debug, Clone)]
pub struct TiledLayerFigure(Arc<TiledLayerFigureData>);

#[derive(Debug)]
struct TiledLayerFigureData {
    id: TiledFigureId,
    offset: Point,
    extent: Extent,
    picture: RwLock<Option<PictureLayer>>,
}

impl TiledLayerFigure {
    pub fn new(id: TiledFigureId, offset: Point, extent: Extent) -> Self {
        Self(Arc::new(TiledLayerFigureData {
            id,
            offset,
            extent,
            picture: Default::default(),
        }))
    }

    pub fn id(&self) -> TiledFigureId {
        self.0.id
    }

    pub fn offset(&self) -> &Point {
        &self.0.offset
    }

    pub fn extent(&self) -> &Extent {
        &self.0.extent
    }

    pub fn top(&self) -> Scalar {
        self.offset().y()
    }

    pub fn left(&self) -> Scalar {
        self.offset().x()
    }

    pub fn right(&self) -> Scalar {
        self.left() + self.extent().width()
    }

    pub fn bottom(&self) -> Scalar {
        self.top() + self.extent().height()
    }

    pub fn has_picture(&self) -> bool {
        self.0.picture.read().is_some()
    }

    pub fn picture(&self) -> Option<impl Layer> {
        self.0
            .picture
            .read()
            .deref()
            .clone()
            .map(|layer| OffsetLayer::wrap_with_offset(layer, self.offset().clone()))
    }

    pub fn get_picture(&self) -> Option<PictureLayer> {
        self.0.picture.read().deref().clone()
    }

    pub fn set_picture(&self, picture: PictureLayer) {
        let _ = self.0.picture.write().insert(picture);
    }

    pub fn with_picture(self, picture: PictureLayer) -> Self {
        self.set_picture(picture);
        self
    }
}

impl RTreeObject for TiledLayerFigure {
    type Envelope = AABB<Point>;

    fn envelope(&self) -> Self::Envelope {
        let corner_1 = self.offset().clone();
        let corner_2 = Point::new(
            self.offset().x() + self.extent().width(),
            self.offset().y() + self.extent().height(),
        );
        AABB::from_corners(corner_1, corner_2)
    }
}

impl Layer for TiledLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_tiled(self)
    }

    fn layers(&self) -> &[Arc<dyn Layer>] {
        &[]
    }

    fn with_layers(&self, _layers: Vec<Arc<dyn Layer>>) -> Arc<dyn Layer> {
        self.clone_arc()
    }

    fn clone_arc(&self) -> Arc<dyn Layer> {
        Arc::new(self.clone())
    }

    fn any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_scale_factor() {
        assert_eq!(
            TiledLayerScaleFactor::scale_in(0.1).zoom_level(),
            Fraction::new(1u64, 10u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(0.25).zoom_level(),
            Fraction::new(1u64, 4u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(0.35).zoom_level(),
            Fraction::new(1u64, 3u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(0.5).zoom_level(),
            Fraction::new(1u64, 2u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(0.75).zoom_level(),
            Fraction::new(1u64, 2u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(1.0).zoom_level(),
            Fraction::new(1u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(1.25).zoom_level(),
            Fraction::new(1u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(1.5).zoom_level(),
            Fraction::new(1u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(1.75).zoom_level(),
            Fraction::new(1u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(2.0).zoom_level(),
            Fraction::new(2u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_in(2.1).zoom_level(),
            Fraction::new(2u64, 1u64)
        );

        assert_eq!(
            TiledLayerScaleFactor::scale_out(0.1).zoom_level(),
            Fraction::new(1u64, 10u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(0.25).zoom_level(),
            Fraction::new(1u64, 4u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(0.35).zoom_level(),
            Fraction::new(1u64, 2u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(0.5).zoom_level(),
            Fraction::new(1u64, 2u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(0.75).zoom_level(),
            Fraction::new(1u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(1.0).zoom_level(),
            Fraction::new(1u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(1.25).zoom_level(),
            Fraction::new(2u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(1.5).zoom_level(),
            Fraction::new(2u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(1.75).zoom_level(),
            Fraction::new(2u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(2.0).zoom_level(),
            Fraction::new(2u64, 1u64)
        );
        assert_eq!(
            TiledLayerScaleFactor::scale_out(2.1).zoom_level(),
            Fraction::new(3u64, 1u64)
        );
    }
}

#[cfg(feature = "phlow")]
mod extensions {
    use phlow::{phlow, phlow_all, PhlowObject, PhlowView};
    use rstar::RTreeNode;

    use super::*;

    #[phlow::extensions(CompositorExtensions, TiledLayer)]
    impl TiledLayerExtensions {
        #[phlow::view]
        pub fn info_for(_this: &TiledLayer, view: impl PhlowView) -> impl PhlowView {
            view.columned_list()
                .title("Info")
                .priority(4)
                .items::<TiledLayer>(|layer| {
                    phlow_all!(vec![
                        ("Camera position", phlow!(layer.camera_position().clone())),
                        ("Viewport extent", phlow!(layer.viewport_extent().clone())),
                        ("Tile extent", phlow!(layer.tile_extent().clone())),
                        (
                            "Columns",
                            phlow!(format!(
                                "{} to {}",
                                layer.left_tile_column(),
                                layer.right_tile_column()
                            ))
                        ),
                        (
                            "Rows",
                            phlow!(format!(
                                "{} to {}",
                                layer.top_tile_row(),
                                layer.bottom_tile_row()
                            ))
                        ),
                        ("RTree root", phlow!(layer.clone_tree_root())),
                    ])
                })
                .column(|column| {
                    column
                        .title("Property")
                        .item::<(&str, PhlowObject)>(|each| phlow!(each.0))
                })
                .column_item::<(&str, PhlowObject)>("Value", |each| phlow!(each.1.to_string()))
                .send::<(&str, PhlowObject)>(|each| each.1.clone())
        }

        #[phlow::view]
        pub fn visible_tiles_for(_this: &TiledLayer, view: impl PhlowView) -> impl PhlowView {
            view.list()
                .title("Visible tiles")
                .priority(5)
                .items::<TiledLayer>(|layer| phlow_all!(layer.visible_tiles()))
        }

        #[phlow::view]
        pub fn visible_figures_for(_this: &TiledLayer, view: impl PhlowView) -> impl PhlowView {
            view.list()
                .title("Visible figures")
                .priority(6)
                .items::<TiledLayer>(|layer| phlow_all!(layer.visible_figures()))
        }

        #[phlow::view]
        pub fn figures_for(_this: &TiledLayer, view: impl PhlowView) -> impl PhlowView {
            view.list()
                .title("All figures")
                .priority(7)
                .items::<TiledLayer>(|layer| phlow_all!(layer.figures()))
        }
    }

    #[phlow::extensions(CompositorExtensions, TiledLayerFigure)]
    impl TiledLayerFigureExtensions {
        #[phlow::view]
        pub fn info_for(_this: &TiledLayerFigure, view: impl PhlowView) -> impl PhlowView {
            view.columned_list()
                .title("Info")
                .priority(4)
                .items::<TiledLayerFigure>(|figure| {
                    phlow_all!(vec![
                        ("Id", phlow!(figure.id())),
                        ("Offset", phlow!(figure.offset().clone())),
                        ("Extent", phlow!(figure.extent().clone())),
                        ("Picture", phlow!(figure.picture()))
                    ])
                })
                .column(|column| {
                    column
                        .title("Property")
                        .item::<(&str, PhlowObject)>(|each| phlow!(each.0))
                })
                .column_item::<(&str, PhlowObject)>("Value", |each| phlow!(each.1.to_string()))
                .send::<(&str, PhlowObject)>(|each| each.1.clone())
        }
    }

    #[phlow::extensions(CompositorExtensions, ParentNode<TiledLayerFigure>)]
    impl ParentNodeExtensions {
        #[phlow::view]
        pub fn info_for(
            _this: &ParentNode<TiledLayerFigure>,
            view: impl PhlowView,
        ) -> impl PhlowView {
            view.columned_list()
                .title("Info")
                .priority(4)
                .items::<ParentNode<TiledLayerFigure>>(
                    |parent_node| phlow_all!(vec![("Envelope", phlow!(parent_node.envelope())),])
                )
                .column(|column| {
                    column
                        .title("Property")
                        .item::<(&str, PhlowObject)>(|each| phlow!(each.0))
                })
                .column_item::<(&str, PhlowObject)>("Value", |each| phlow!(each.1.to_string()))
                .send::<(&str, PhlowObject)>(|each| each.1.clone())
        }

        #[phlow::view]
        pub fn children_for(
            _this: &ParentNode<TiledLayerFigure>,
            view: impl PhlowView,
        ) -> impl PhlowView {
            view.list()
                .title("Children")
                .priority(5)
                .items::<ParentNode<TiledLayerFigure>>(|parent_node| {
                    let layers = parent_node
                        .children()
                        .iter()
                        .map(|node| node.clone())
                        .clone();
                    phlow_all!(layers)
                })
                .item_text::<RTreeNode<TiledLayerFigure>>(|each| each.to_string())
        }
    }

    #[phlow::extensions(CompositorExtensions, RTreeNode<TiledLayerFigure>)]
    impl RTreeNodeExtensions {
        #[phlow::view]
        pub fn children_for(
            _this: &RTreeNode<TiledLayerFigure>,
            view: impl PhlowView,
        ) -> impl PhlowView {
            view.list()
                .title("Children")
                .priority(5)
                .items::<RTreeNode<TiledLayerFigure>>(|node| {
                    let children = match node.as_ref() {
                        RTreeNode::Leaf(_) => {
                            vec![]
                        }
                        RTreeNode::Parent(parent_node) => parent_node
                            .children()
                            .iter()
                            .map(|node| node.clone())
                            .collect::<Vec<RTreeNode<TiledLayerFigure>>>(),
                    };

                    phlow_all!(children)
                })
                .item_text::<RTreeNode<TiledLayerFigure>>(|each| each.to_string())
        }
    }
}
