use crate::{Compositor, Geometry, Layer, Matrix, Point};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct StateCommand {
    pub command_type: StateCommandType,
    pub offset: Point,
}

#[derive(Debug, Clone)]
pub enum StateCommandType {
    Transform(Matrix),
    Clip(Geometry),
}

impl StateCommand {
    pub fn clip(geometry: Geometry, offset: Point) -> Self {
        StateCommand {
            command_type: StateCommandType::Clip(geometry),
            offset,
        }
    }

    pub fn transform(matrix: Matrix, offset: Point) -> Self {
        StateCommand {
            command_type: StateCommandType::Transform(matrix),
            offset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LeftoverStateLayer {
    layers: Vec<Arc<dyn Layer>>,
    pub commands: Vec<StateCommand>,
}

impl LeftoverStateLayer {
    pub fn new(commands: Vec<StateCommand>) -> Self {
        Self {
            layers: vec![],
            commands,
        }
    }

    pub fn clip(&mut self, geometry: Geometry, offset: Point) {
        self.commands.push(StateCommand::clip(geometry, offset));
    }
}

impl Layer for LeftoverStateLayer {
    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_leftover(self);
    }

    fn layers(&self) -> &[Arc<dyn Layer>] {
        &self.layers
    }

    fn clone_arc(&self) -> Arc<dyn Layer> {
        Arc::new(self.clone())
    }

    fn any(&self) -> &dyn Any {
        self
    }
}
