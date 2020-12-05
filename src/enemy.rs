use crate::animation::{Animated, AnimationState, CardinalDirection};
use crate::builder::Builder;
use bevy::prelude::*;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::rapier::dynamics::{RigidBody, RigidBodySet};
use bevy_rapier3d::rapier::na;
use bevy_rapier3d::rapier::na::{Point3, Isometry3, Vector3};

const ENEMY_SPEED: f32 = 30.;

// distances are squared to simplify operations
const ENEMY_SEARCH_DISTANCE: f32 = 22500.;
const ENEMY_ATTACK_DISTANCE: f32 = 625.;

pub struct Enemy {
    pub hp: i32,
    pub name: String,
}

impl Enemy {
    pub fn new(hp: i32, name: String) -> Self {
        Enemy { hp, name }
    }

    pub fn generic_enemy() -> Self {
        Enemy::new(80, String::from("generic enemy"))
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
}

pub fn move_enemies(
    mut rigid_body_set: ResMut<RigidBodySet>,
    builder_query: Query<(&Builder, &RigidBodyHandleComponent)>,
    mut enemy_query: Query<(&mut Animated, &Enemy, &RigidBodyHandleComponent)>,
) {
    let mut builder_positions: Vec<Isometry3<f32>> = Vec::new();
    for (_builder, rigid_body_handle) in builder_query.iter() {
        builder_positions.push(
            rigid_body_set.get(rigid_body_handle.handle()).unwrap().position().clone()
        );
    }

    for (mut animated, _enemy, rigid_body_handle) in enemy_query.iter_mut() {
        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();
        // if enemy is still, look for hero if close enough nearby and walk toward them
        if animated.state == AnimationState::Idle {
            let mut reset_enemy = true;
            for builder_position in &builder_positions {
                let distance = distance_to_position(rigid_body, &builder_position);
                // println!("Enemy distance {}", distance);
                if distance < ENEMY_SEARCH_DISTANCE {
                    animated.state = AnimationState::Move;
                    animated.animation_index = 0;

                    let movement_direction =
                        direction_to_position(rigid_body, &builder_position) * ENEMY_SPEED;

                    if movement_direction.x < 0. {
                        animated.facing = CardinalDirection::West;
                    } else {
                        animated.facing = CardinalDirection::East;
                    }
                    rigid_body.set_linvel(movement_direction, false);

                    if distance < ENEMY_ATTACK_DISTANCE {
                        animated.state = AnimationState::Attack;
                        animated.animation_index = 0;
                    }
                    reset_enemy = false;
                }
            }

            if reset_enemy {
                rigid_body.set_linvel(Vector3::zeros(), false);
            }
        }
    }
}

/// Returns the squared distance between the builder and enemy.
///
/// sqrt operation (for correctness) is skipped to avoid an extra operation
fn distance_to_position(rigid_body: &RigidBody, position: &Isometry3<f32>) -> f32 {
    let body_point = Point3::from(rigid_body.position().translation.vector);
    let position_point = Point3::from(position.translation.vector);

    na::distance_squared(&body_point, &position_point)
}

fn direction_to_position(rigid_body: &RigidBody, position: &Isometry3<f32>) -> Vector3<f32> {

    (position.translation.vector - rigid_body.position().translation.vector).normalize()
}
