use crate::{Clip, Compositor, Layer, Matrix, Path, Point, Rectangle, RoundedRectangle};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct StateCommand {
    pub command_type: StateCommandType,
    pub offset: Point,
}

#[derive(Debug, Clone)]
pub enum StateCommandType {
    Transform(Matrix),
    Clip(Clip),
}

impl StateCommand {
    pub fn clip_rect(rect: Rectangle, offset: Point) -> Self {
        StateCommand {
            command_type: StateCommandType::Clip(Clip::Rectangle(rect)),
            offset,
        }
    }

    pub fn clip_rrect(rrect: RoundedRectangle, offset: Point) -> Self {
        StateCommand {
            command_type: StateCommandType::Clip(Clip::RoundedRectangle(rrect)),
            offset,
        }
    }

    pub fn clip_path(path: Path, offset: Point) -> Self {
        StateCommand {
            command_type: StateCommandType::Clip(Clip::Path(path)),
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

#[derive(Debug)]
pub struct LeftoverStateLayer {
    layers: Vec<Arc<dyn Layer>>,
    pub commands: Vec<StateCommand>,
}

impl LeftoverStateLayer {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            commands: vec![],
        }
    }

    pub fn clip_rect(&mut self, rect: Rectangle, offset: Point) {
        self.commands.push(StateCommand::clip_rect(rect, offset));
    }

    pub fn clip_rrect(&mut self, rrect: RoundedRectangle, offset: Point) {
        self.commands.push(StateCommand::clip_rrect(rrect, offset));
    }

    pub fn clip_path(&mut self, path: Path, offset: Point) {
        self.commands.push(StateCommand::clip_path(path, offset));
    }

    pub fn transform(&mut self, matrix: Matrix, offset: Point) {
        self.commands.push(StateCommand::transform(matrix, offset));
    }
}

impl Layer for LeftoverStateLayer {
    fn layers(&self) -> &[Arc<dyn Layer>] {
        &self.layers
    }

    fn compose(&self, compositor: &mut dyn Compositor) {
        compositor.compose_leftover(self);
    }
}
