use bevy::prelude::*;

use crate::map_generator::{Builder, BuilderState};

const WIZARD_SPEED: f32 = 3.;

pub fn control_builder(
    keyboard_input: Res<Input<KeyCode>>,
    mut builder: Mut<Builder>,
    mut translation: Mut<Translation>,
    mut timer: Mut<Timer>
) {
    if keyboard_input.pressed(KeyCode::W) {
        (* translation.0.y_mut()) += WIZARD_SPEED;
    }
    if keyboard_input.pressed(KeyCode::S) {
        (* translation.0.y_mut()) -= WIZARD_SPEED;
    }
    if keyboard_input.pressed(KeyCode::A) {
        (* translation.0.x_mut()) -= WIZARD_SPEED;
    }
    if keyboard_input.pressed(KeyCode::D) {
        (* translation.0.x_mut()) += WIZARD_SPEED;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        builder.state = BuilderState::Attack;
        builder.animation_index = 0;
        timer.reset();
        timer.finished = true;
    }
}