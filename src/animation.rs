use bevy::prelude::*;
use bevy::core::Timer;
use crate::data;
use bevy::ecs::Query;
use bevy::sprite::TextureAtlasSprite;
use bevy_rapier3d::rapier::dynamics::RigidBodySet;
use bevy_rapier3d::rapier::math::{AngVector, Rotation, Vector};
use bevy_rapier3d::physics::RigidBodyHandleComponent;

const ANIMATION_SPEED: f32 = 0.5;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum AnimationState {
    Idle,
    Move,
    Attack,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum UnitType {
    Wizard,
    Enemy,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum CardinalDirection {
    North,
    South,
    West,
    East,
}

pub struct Animated {
    pub unit_type: UnitType,
    pub state: AnimationState,
    pub animation_index: u32,
    pub facing: CardinalDirection,
}

impl Animated {
    fn new(unit_type: UnitType) -> Self {
        Animated {
            unit_type,
            state: AnimationState::Idle,
            animation_index: 0,
            facing: CardinalDirection::East,
        }
    }
}

#[derive(Bundle)]
pub struct AnimationBundle {
    pub animated: Animated,
    pub timer: Timer,
}

impl AnimationBundle {
    pub fn new(unit_type: UnitType) -> Self {
        let animation_info = data::get_animation_info(&unit_type, &AnimationState::Idle);
        AnimationBundle {
            animated: Animated::new(unit_type),
            timer: Timer::from_seconds(animation_info.durations[0], false),
        }
    }
}

pub fn animate(
    mut rigid_body_set: ResMut<RigidBodySet>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Animated, &RigidBodyHandleComponent)>
) {
    for (mut timer, mut sprite, mut animated, rigid_body_handle) in query.iter_mut() {
        if timer.finished {
            let mut animation_info =
                data::get_animation_info(&animated.unit_type, &animated.state);

            let next_index = animated.animation_index + 1;
            if next_index >= animation_info.length {
                if animation_info.loop_around {
                    animated.animation_index = next_index % animation_info.length
                } else {
                    animated.animation_index = 0;
                    animated.state = AnimationState::Idle;
                    animation_info =
                        data::get_animation_info(&animated.unit_type, &AnimationState::Idle);
                }
            } else {
                animated.animation_index = next_index;
            }

            sprite.index = animation_info.sprite_offset + animated.animation_index;

            timer.reset();
            timer.duration =
                animation_info.durations[animated.animation_index as usize] * ANIMATION_SPEED;
        }

        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();
        let mut previous_position = rigid_body.position().clone();
        if animated.facing == CardinalDirection::West {
            previous_position.rotation =
                Rotation::new(AngVector::new(0.0, std::f32::consts::PI, 0.0));
        } else if animated.facing == CardinalDirection::East {
            previous_position.rotation = Rotation::new(AngVector::new(0.0, 0.0, 0.0));
        }
        rigid_body.set_position(previous_position, true);
    }
}

