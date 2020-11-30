use bevy::prelude::*;

use crate::builder::{Builder, BuilderState, CardinalDirection};
use bevy::render::camera::Camera;

const WIZARD_SPEED: f32 = 10.;

pub fn control_builder(
    keyboard_input: Res<Input<KeyCode>>,
    mut query_builder: Query<(&mut Timer, &mut Builder, &mut Transform)>,
    mut query_camera: Query<(&Camera, &mut Transform)>,
) {
    let query_builder_iterator = &mut query_builder.iter_mut();
    let query_camera_iterator = &mut query_camera.iter_mut();

    // TODO: should be able to rework this?

    if let Some((mut builder_timer, mut builder, mut builder_transform)) =
        query_builder_iterator.into_iter().next()
    {
        if let Some((_camera, mut camera_transform)) = query_camera_iterator.into_iter().next() {
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
                    (*builder_transform.translation.y_mut()) += WIZARD_SPEED;
                }
                if press_down {
                    (*builder_transform.translation.y_mut()) -= WIZARD_SPEED;
                }
                if press_left {
                    (*builder_transform.translation.x_mut()) -= WIZARD_SPEED;
                    if builder.facing == CardinalDirection::East {
                        builder.facing = CardinalDirection::West;
                        builder_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                    }
                }
                if press_right {
                    (*builder_transform.translation.x_mut()) += WIZARD_SPEED;
                    if builder.facing == CardinalDirection::West {
                        builder.facing = CardinalDirection::East;
                        builder_transform.rotation = Quat::default();
                    }
                }
            }

            (*camera_transform.translation.x_mut()) = builder_transform.translation.x();
            (*camera_transform.translation.y_mut()) = builder_transform.translation.y();

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
