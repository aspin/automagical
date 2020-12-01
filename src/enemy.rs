use bevy::prelude::*;
use crate::builder::{Animated, Builder};
use bevy_rapier3d::rapier::dynamics::{RigidBody, RigidBodySet};
use crate::data::animation::AnimationState;
use bevy_rapier3d::rapier::math::Vector;
use bevy_rapier3d::physics::RigidBodyHandleComponent;

pub const ENEMY_SPEED: f32 = 30.;

pub struct Enemy {
    pub hp: i32,
    pub name: String
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
    mut enemy_query: Query<(&mut Animated, &Enemy, &RigidBodyHandleComponent)>
) {
    let mut builder_positions: Vec<&Transform> = Vec::new();
    for (_builder, transform) in builder_query.iter() {
        builder_positions.push(transform);
    }

    for (mut animated, _enemy, rigid_body_handle) in enemy_query.iter_mut() {
        let mut rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();
        // if enemy is still, look for hero if close enough nearby and walk toward them
        if animated.state == AnimationState::Idle {
            for builder_position in &builder_positions {
                let distance = builder_enemy_distance(builder_position, &*rigid_body);
                // println!("Enemy distance {}", distance);
                if distance < 150. {
                    animated.state = AnimationState::Move;
                    animated.animation_index = 0;

                    let movement_direction = get_builder_direction(
                        builder_position, &*rigid_body
                    ) * ENEMY_SPEED;

                    rigid_body.linvel = Vector::new(
                        movement_direction.x(), movement_direction.y(), movement_direction.z()
                    );
                    rigid_body.wake_up(false);

                    if distance < 25. {
                        animated.state = AnimationState::Attack;
                        animated.animation_index = 0;
                    }
                } else {
                    rigid_body.linvel = Vector::new(0., 0., 0.);
                }
            }
        }
    }
}

fn builder_enemy_distance(builder_transform: &Transform, enemy_rigid_body: &RigidBody) -> f32 {
    let (builder_x, builder_y, builder_z) = (
        builder_transform.translation.x(),
        builder_transform.translation.y(),
        builder_transform.translation.z()
    );
    let enemy_x: f32 = enemy_rigid_body.position.translation.x;
    let enemy_y: f32 = enemy_rigid_body.position.translation.y;
    let enemy_z: f32 = enemy_rigid_body.position.translation.z;

    ((builder_x - enemy_x).powi(2) + (builder_y - enemy_y).powi(2) + (builder_z - enemy_z).powi(2)).sqrt()
}

fn get_builder_direction(builder_transform: &Transform, enemy_rigid_body: &RigidBody) -> Vec3 {
    let (builder_x, builder_y) = (
        builder_transform.translation.x(),
        builder_transform.translation.y(),
    );
    let enemy_x: f32 = enemy_rigid_body.position.translation.x;
    let enemy_y: f32 = enemy_rigid_body.position.translation.y;

    Vec3::new(
        builder_x - enemy_x,
        builder_y - enemy_y,
        0.,
    ).normalize()
}