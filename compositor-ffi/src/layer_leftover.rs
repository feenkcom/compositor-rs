use compositor::{Geometry, Layer, LeftoverStateLayer, Matrix, Point, StateCommand};
use std::sync::Arc;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn compositor_leftover_clip_command(
    geometry: OwnedPtr<Geometry>,
    offset_x: f32,
    offset_y: f32,
) -> OwnedPtr<StateCommand> {
    geometry
        .with_value_ok(|geometry| {
            OwnedPtr::new(StateCommand::clip(
                geometry,
                Point::new_f32(offset_x, offset_y),
            ))
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_leftover_transform_command(
    matrix: OwnedPtr<Matrix>,
    offset_x: f32,
    offset_y: f32,
) -> OwnedPtr<StateCommand> {
    matrix
        .with_value_ok(|matrix| {
            OwnedPtr::new(StateCommand::transform(
                matrix,
                Point::new_f32(offset_x, offset_y),
            ))
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_leftover_command_drop(command: OwnedPtr<StateCommand>) {
    drop(command);
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_leftover_commands_new() -> OwnedPtr<Vec<StateCommand>> {
    OwnedPtr::new(vec![])
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_leftover_commands_add(
    mut commands: BorrowedPtr<Vec<StateCommand>>,
    command: OwnedPtr<StateCommand>,
) {
    commands
        .with_mut(|commands| {
            command.with_value_ok(|command| {
                commands.push(command);
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_leftover_commands_drop(commands: OwnedPtr<Vec<StateCommand>>) {
    drop(commands);
}

#[unsafe(no_mangle)]
pub extern "C" fn compositor_leftover_layer_new(
    commands: OwnedPtr<Vec<StateCommand>>,
) -> OwnedPtr<Arc<dyn Layer>> {
    commands
        .with_value_ok(|commands| {
            OwnedPtr::new(Arc::new(LeftoverStateLayer::new(commands)) as Arc<dyn Layer>)
        })
        .or_log(OwnedPtr::null())
}
