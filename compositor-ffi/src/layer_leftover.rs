use compositor::{Geometry, Layer, LeftoverStateLayer, Matrix, Point, StateCommand};
use std::sync::Arc;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn compositor_leftover_clip_command(
    geometry: *mut ValueBox<Geometry>,
    offset_x: f32,
    offset_y: f32,
) -> *mut ValueBox<StateCommand> {
    geometry
        .take_value()
        .map(|geometry| StateCommand::clip(geometry, Point::new_f32(offset_x, offset_y)))
        .into_raw()
}

#[no_mangle]
pub fn compositor_leftover_transform_command(
    mut matrix: *mut ValueBox<Matrix>,
    offset_x: f32,
    offset_y: f32,
) -> *mut ValueBox<StateCommand> {
    matrix
        .take_value()
        .map(|geometry| StateCommand::transform(geometry, Point::new_f32(offset_x, offset_y)))
        .into_raw()
}

#[no_mangle]
pub fn compositor_leftover_command_drop(command: *mut ValueBox<StateCommand>) {
    command.release();
}

#[no_mangle]
pub fn compositor_leftover_commands_new() -> *mut ValueBox<Vec<StateCommand>> {
    ValueBox::new(vec![]).into_raw()
}

#[no_mangle]
pub fn compositor_leftover_commands_add(
    commands: *mut ValueBox<Vec<StateCommand>>,
    mut command: *mut ValueBox<StateCommand>,
) {
    commands
        .with_mut(|commands| {
            command.take_value().map(|command| {
                commands.push(command);
            })
        })
        .log();
}

#[no_mangle]
pub fn compositor_leftover_commands_drop(commands: *mut ValueBox<Vec<StateCommand>>) {
    commands.release();
}

#[no_mangle]
pub fn compositor_leftover_layer_new(
    commands: *mut ValueBox<Vec<StateCommand>>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    commands
        .take_value()
        .map(|commands| (Arc::new(LeftoverStateLayer::new(commands)) as Arc<dyn Layer>))
        .into_raw()
}
