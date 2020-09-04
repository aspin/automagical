use bevy::prelude::*;

use crate::map_generator::{Builder, BuilderState};
use bevy::render::camera::Camera;

const WIZARD_SPEED: f32 = 3.;

pub fn control_builder(
    keyboard_input: Res<Input<KeyCode>>,
    mut query_builder: Query<(&mut Timer, &mut Builder, &mut Translation)>,
    mut query_camera: Query<(&Camera, &mut Translation)>,
) {

    let query_builder_iterator = &mut query_builder.iter();
    let query_camera_iterator = &mut query_camera.iter();

    if let Some((mut builder_timer, mut builder, mut builder_translation)) = query_builder_iterator.into_iter().next() {
        if let Some((_camera, mut camera_translation)) = query_camera_iterator.into_iter().next() {
            let press_up = keyboard_input.pressed(KeyCode::W);
            let press_down = keyboard_input.pressed(KeyCode::S);
            let press_left = keyboard_input.pressed(KeyCode::A);
            let press_right = keyboard_input.pressed(KeyCode::D);

            if press_up || press_down || press_left || press_right {
                if builder.state == BuilderState::Idle {
                    builder.state = BuilderState::Move;
                    builder.animation_index = 0;
                    builder_timer.reset();
                    builder_timer.finished = true;
                }

                if press_up {
                    (* builder_translation.0.y_mut()) += WIZARD_SPEED;
                }
                if press_down {
                    (* builder_translation.0.y_mut()) -= WIZARD_SPEED;
                }
                if press_left {
                    (* builder_translation.0.x_mut()) -= WIZARD_SPEED;
                }
                if press_right {
                    (* builder_translation.0.x_mut()) += WIZARD_SPEED;
                }
            }

            (* camera_translation.0.x_mut()) = builder_translation.0.x();
            (* camera_translation.0.y_mut()) = builder_translation.0.y();

            if keyboard_input.pressed(KeyCode::Space) {
                if builder.state != BuilderState::Attack {
                    builder.state = BuilderState::Attack;
                    builder.animation_index = 0;
                    builder_timer.reset();
                    builder_timer.finished = true;
                }
            }
        }
    }
}