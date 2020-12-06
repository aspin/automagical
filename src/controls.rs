use bevy::prelude::*;

use crate::animation::{Animated, AnimationState, CardinalDirection};
use crate::builder::{Builder, BuilderMode};
use bevy::render::camera::Camera;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::rapier::dynamics::RigidBodySet;
use bevy_rapier3d::rapier::math::Vector;
use crate::cursor::CursorState;

const WIZARD_SPEED: f32 = 100.;

pub fn control_builder(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut rigid_body_set: ResMut<RigidBodySet>,
    cursor_state: Res<CursorState>,
    mut query_builder: Query<(
        &mut Timer,
        &mut Builder,
        &mut Animated,
        &RigidBodyHandleComponent,
    )>,
    mut query_camera: Query<(&Camera, &mut Transform)>,
) {
    let query_builder_iterator = &mut query_builder.iter_mut();
    let query_camera_iterator = &mut query_camera.iter_mut();

    if let Some((mut builder_timer, mut builder, mut animated, builder_body_handle)) =
        query_builder_iterator.next()
    {
        let builder_body = rigid_body_set
            .get_mut(builder_body_handle.handle())
            .unwrap();
        if let Some((_camera, mut camera_transform)) = query_camera_iterator.next() {
            let press_up = keyboard_input.pressed(KeyCode::W);
            let press_down = keyboard_input.pressed(KeyCode::S);
            let press_left = keyboard_input.pressed(KeyCode::A);
            let press_right = keyboard_input.pressed(KeyCode::D);

            if press_up || press_down || press_left || press_right {
                if animated.state == AnimationState::Idle {
                    animated.state = AnimationState::Move;
                    animated.animation_index = 0;
                    builder_timer.reset();
                    builder_timer.finished = true;
                }

                let mut x_speed = 0.;
                let mut y_speed = 0.;
                if press_up {
                    y_speed = WIZARD_SPEED;
                }
                if press_down {
                    y_speed = -WIZARD_SPEED;
                }
                if press_left {
                    x_speed = -WIZARD_SPEED;
                    animated.facing = CardinalDirection::West;
                }
                if press_right {
                    x_speed = WIZARD_SPEED;
                    animated.facing = CardinalDirection::East;
                }
                builder_body.set_linvel(Vector::new(x_speed, y_speed, 0.), true);
            } else {
                builder_body.set_linvel(Vector::zeros(), true);
            }

            (*camera_transform.translation.x_mut()) = builder_body.position().translation.x;
            (*camera_transform.translation.y_mut()) = builder_body.position().translation.y;
        }

        // toggle build mode
        if mouse_button_input.just_released(MouseButton::Right) {
            builder.toggle_mode();
            println!("Setting builder mode: {:?}", builder.mode)
        }

        // fire projectiles
        if mouse_button_input.pressed(MouseButton::Left)
            && builder.mode == BuilderMode::Combat
            && animated.state != AnimationState::Attack {

            if let Some(cursor_coordinates) = cursor_state.cursor_position {
                animated.state = AnimationState::Attack;
                animated.animation_index = 0;
                builder_timer.reset();
                builder_timer.finished = true;
                builder.aim_location.replace(cursor_coordinates);
            }
        }
    }
}
