use crate::animation::{Animated, AnimationState, CardinalDirection};
use crate::builder::Builder;
use bevy::prelude::*;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::rapier::dynamics::{RigidBody, RigidBodySet};
use bevy_rapier3d::rapier::math::Vector;

const ENEMY_SPEED: f32 = 30.;
const ENEMY_SEARCH_DISTANCE: f32 = 150.;
const ENEMY_ATTACK_DISTANCE: f32 = 25.;

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
    builder_query: Query<(&Builder, &Transform)>,
    mut enemy_query: Query<(&mut Animated, &Enemy, &RigidBodyHandleComponent)>,
) {
    let mut builder_positions: Vec<&Transform> = Vec::new();
    for (_builder, transform) in builder_query.iter() {
        builder_positions.push(transform);
    }

    for (mut animated, _enemy, rigid_body_handle) in enemy_query.iter_mut() {
        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();
        // if enemy is still, look for hero if close enough nearby and walk toward them
        if animated.state == AnimationState::Idle {
            for builder_position in &builder_positions {
                let distance = builder_enemy_distance(builder_position, &*rigid_body);
                // println!("Enemy distance {}", distance);
                if distance < ENEMY_SEARCH_DISTANCE {
                    animated.state = AnimationState::Move;
                    animated.animation_index = 0;

                    let movement_direction =
                        get_builder_direction(builder_position, &*rigid_body) * ENEMY_SPEED;

                    if movement_direction.x() < 0. {
                        animated.facing = CardinalDirection::West;
                    } else {
                        animated.facing = CardinalDirection::East;
                    }
                    rigid_body.set_linvel(
                        Vector::new(
                            movement_direction.x(),
                            movement_direction.y(),
                            movement_direction.z(),
                        ),
                        false,
                    );

                    if distance < ENEMY_ATTACK_DISTANCE {
                        animated.state = AnimationState::Attack;
                        animated.animation_index = 0;
                    }
                } else {
                    rigid_body.set_linvel(Vector::zeros(), false);
                }
            }
        }
    }
}

fn builder_enemy_distance(builder_transform: &Transform, enemy_rigid_body: &RigidBody) -> f32 {
    let builder_x = builder_transform.translation.x();
    let builder_y = builder_transform.translation.y();
    let builder_z = builder_transform.translation.z();
    let enemy_x: f32 = enemy_rigid_body.position().translation.x;
    let enemy_y: f32 = enemy_rigid_body.position().translation.y;
    let enemy_z: f32 = enemy_rigid_body.position().translation.z;

    // TODO: remove sqrt to reduce an operation
    ((builder_x - enemy_x).powi(2) + (builder_y - enemy_y).powi(2) + (builder_z - enemy_z).powi(2))
        .sqrt()
}

fn get_builder_direction(builder_transform: &Transform, enemy_rigid_body: &RigidBody) -> Vec3 {
    let (builder_x, builder_y) = (
        builder_transform.translation.x(),
        builder_transform.translation.y(),
    );
    let enemy_x: f32 = enemy_rigid_body.position().translation.x;
    let enemy_y: f32 = enemy_rigid_body.position().translation.y;

    Vec3::new(builder_x - enemy_x, builder_y - enemy_y, 0.).normalize()
}
