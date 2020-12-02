use bevy::prelude::*;

use crate::builder::{Animated, Builder, CardinalDirection};
use crate::animation::AnimationState;
use bevy::render::camera::Camera;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::rapier::dynamics::RigidBodySet;
use bevy_rapier3d::rapier::math::{AngVector, Rotation, Vector};

const WIZARD_SPEED: f32 = 100.;

pub fn control_builder(
    keyboard_input: Res<Input<KeyCode>>,
    mut rigid_body_set: ResMut<RigidBodySet>,
    mut query_builder: Query<(
        &mut Timer,
        &Builder,
        &mut Animated,
        &RigidBodyHandleComponent,
    )>,
    mut query_camera: Query<(&Camera, &mut Transform)>,
) {
    let query_builder_iterator = &mut query_builder.iter_mut();
    let query_camera_iterator = &mut query_camera.iter_mut();

    // TODO: should be able to rework this?

    if let Some((mut builder_timer, _builder, mut animated, builder_body_handle)) =
        query_builder_iterator.into_iter().next()
    {
        let builder_body = rigid_body_set
            .get_mut(builder_body_handle.handle())
            .unwrap();
        if let Some((_camera, mut camera_transform)) = query_camera_iterator.into_iter().next() {
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

                if press_up {
                    builder_body.set_linvel(Vector::new(0., WIZARD_SPEED, 0.), true);
                }
                if press_down {
                    builder_body.set_linvel(Vector::new(0., -WIZARD_SPEED, 0.), true);
                }
                if press_left {
                    builder_body.set_linvel(Vector::new(-WIZARD_SPEED, 0., 0.), true);
                    if animated.facing == CardinalDirection::East {
                        animated.facing = CardinalDirection::West;

                        let mut previous_position = builder_body.position().clone();
                        previous_position.rotation =
                            Rotation::new(AngVector::new(0.0, std::f32::consts::PI, 0.0));
                        builder_body.set_position(previous_position, true);
                    }
                }
                if press_right {
                    builder_body.set_linvel(Vector::new(WIZARD_SPEED, 0., 0.), true);
                    if animated.facing == CardinalDirection::West {
                        animated.facing = CardinalDirection::East;
                        let mut previous_position = builder_body.position().clone();
                        previous_position.rotation = Rotation::new(AngVector::new(0.0, 0.0, 0.0));
                        builder_body.set_position(previous_position, true);
                    }
                }
            } else {
                builder_body.set_linvel(Vector::zeros(), true);
            }

            (*camera_transform.translation.x_mut()) = builder_body.position().translation.x;
            (*camera_transform.translation.y_mut()) = builder_body.position().translation.y;

            if keyboard_input.pressed(KeyCode::Space) {
                if animated.state != AnimationState::Attack {
                    animated.state = AnimationState::Attack;
                    animated.animation_index = 0;
                    builder_timer.reset();
                    builder_timer.finished = true;
                }
            }
        }
    }
}
