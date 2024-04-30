use crate::{Compositor, Layer, Point};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OffsetLayer {
    layers: Vec<Arc<dyn Layer>>,
    offset: Point,
}

impl OffsetLayer {
    pub fn new() -> Self {
        Self::new_offset(Point::zero())
    }

    pub fn new_offset(offset: Point) -> Self {
        Self {
            layers: vec![],
            offset,
        }
    }

    pub fn wrap_with_offset(layer: impl Layer, offset: Point) -> Self {
        Self {
            layers: vec![layer.clone_arc()],
            offset,
        }
    }

    pub fn offset(&self) -> &Point {
        &self.offset
    }

    /// Create a new offset layer with a given offset preserving the sub-layers
    pub fn with_offset(&self, offset: Point) -> Self {
        Self {
            layers: self.layers.clone(),
            offset,
        }
    }
}

impl Layer for OffsetLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_offset(self);
    }

    fn layers(&self) -> &[Arc<dyn Layer>] {
        self.layers.as_slice()
    }

    fn with_layers(&self, layers: Vec<Arc<dyn Layer>>) -> Arc<dyn Layer> {
        Arc::new(Self {
            layers,
            offset: self.offset.clone(),
        })
    }

    fn clone_arc(&self) -> Arc<dyn Layer> {
        Arc::new(self.clone())
    }

    fn any(&self) -> &dyn Any {
        self
    }
}

#[cfg(feature = "phlow")]
mod extensions {
    use super::*;
    use phlow::{phlow, phlow_all, PhlowObject, PhlowView};

    #[phlow::extensions(CompositorExtensions, OffsetLayer)]
    impl OffsetLayerExtensions {
        #[phlow::view]
        pub fn layers_for(_this: &OffsetLayer, view: impl PhlowView) -> impl PhlowView {
            view.list()
                .title("Layers")
                .priority(5)
                .items::<OffsetLayer>(|layer| {
                    let layers = layer.layers.clone();
                    phlow_all!(layers)
                })
                .item_text::<&Arc<dyn Layer>>(|each| each.to_string())
        }

        #[phlow::view]
        pub fn info_for(_this: &OffsetLayer, view: impl PhlowView) -> impl PhlowView {
            view.columned_list()
                .title("Info")
                .priority(4)
                .items::<OffsetLayer>(|layer| {
                    phlow_all!(vec![("Offset", phlow!(layer.offset.clone())),])
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let offset = OffsetLayer::new();
        assert_eq!(offset.offset, Point::zero());
        assert_eq!(offset.count_layers(), 0);
    }

    #[test]
    fn test_new_offset() {
        let offset = OffsetLayer::new_offset(Point::new_f32(20.0, 10.0));
        assert_eq!(offset.offset, Point::new_f32(20.0, 10.0));
        assert_eq!(offset.count_layers(), 0);
    }

    #[test]
    fn test_with_offset() {
        let offset = OffsetLayer::new().with_offset(Point::new_f32(10.0, 20.0));
        assert_eq!(offset.offset, Point::new_f32(10.0, 20.0));
    }

    #[test]
    fn test_arc_with_offset() {
        let layer = Arc::new(OffsetLayer::new());
        let layer_with_offset = layer.with_offset(Point::new_f32(10.0, 20.0));
        assert_eq!(layer_with_offset.offset, Point::new_f32(10.0, 20.0));
    }

    #[test]
    fn test_new_dyn_object() {
        let layer: Arc<dyn Layer> = Arc::new(OffsetLayer::new());
        assert_eq!(layer.count_layers(), 0);
    }
}
