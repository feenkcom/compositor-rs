use boxer::{ValueBox, ValueBoxPointer};
use compositor::{Geometry, Layer, LeftoverStateLayer, Matrix, Point, StateCommand};
use std::sync::Arc;

#[no_mangle]
pub fn compositor_leftover_clip_command(
    mut geometry: *mut ValueBox<Geometry>,
    offset_x: f32,
    offset_y: f32,
) -> *mut ValueBox<StateCommand> {
    geometry.with_not_null_value_consumed_return(std::ptr::null_mut(), |geometry| {
        ValueBox::new(StateCommand::clip(
            geometry,
            Point::new_f32(offset_x, offset_y),
        ))
        .into_raw()
    })
}

#[no_mangle]
pub fn compositor_leftover_transform_command(
    mut matrix: *mut ValueBox<Matrix>,
    offset_x: f32,
    offset_y: f32,
) -> *mut ValueBox<StateCommand> {
    matrix.with_not_null_value_consumed_return(std::ptr::null_mut(), |geometry| {
        ValueBox::new(StateCommand::transform(
            geometry,
            Point::new_f32(offset_x, offset_y),
        ))
        .into_raw()
    })
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
    commands.with_not_null(|commands| {
        command.with_not_null_value_consumed(|command| {
            commands.push(command);
        })
    })
}

#[no_mangle]
pub fn compositor_leftover_commands_drop(commands: *mut ValueBox<Vec<StateCommand>>) {
    commands.release();
}

#[no_mangle]
pub fn compositor_leftover_layer_new(
    mut commands: *mut ValueBox<Vec<StateCommand>>,
) -> *mut ValueBox<Arc<dyn Layer>> {
    commands.with_not_null_value_consumed_return(std::ptr::null_mut(), |commands| {
        ValueBox::new(Arc::new(LeftoverStateLayer::new(commands)) as Arc<dyn Layer>).into_raw()
    })
}
