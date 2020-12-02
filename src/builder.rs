use crate::asset_loader::AtlasHandles;
use crate::projectile::{Projectile, ARROW_SPEED};
use bevy::prelude::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::rapier::math::AngVector;
use crate::animation::{UnitType, AnimationState};
use crate::data;

const ANIMATION_SPEED: f32 = 0.5;

pub struct Builder {
    pub name: String,
}

impl Builder {
    pub fn new(name: &str) -> Self {
        Builder {
            name: String::from(name),
        }
    }
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

#[derive(PartialEq, Copy, Clone)]
pub enum CardinalDirection {
    North,
    South,
    West,
    East,
}

pub fn animate(mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Animated)>) {
    for (mut timer, mut sprite, mut animated) in query.iter_mut() {
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
    }
}

pub fn produce_projectiles(
    mut commands: Commands,
    atlas_handles: Res<AtlasHandles>,
    animated: &Animated,
    builder_transform: &Transform,
    _builder: &Builder,
) {
    if let Some(arrow_id) = atlas_handles.arrow_id {
        if animated.state == AnimationState::Attack && animated.animation_index == 3 {
            for i in 0..3 {
                let arrow_atlas_handle = Handle::weak(arrow_id);
                let projectile = Projectile::arrow(animated.facing.clone());
                let projectile_timer = Timer::from_seconds(projectile.ttl, false);

                let mut x_offset: f32 = 16.;
                let mut x_velocity: f32 = ARROW_SPEED;
                if animated.facing == CardinalDirection::West {
                    x_offset *= -1.;
                    x_velocity *= -1.;
                }
                let y_offset = -4.;
                let y_width = 4.;

                let x = builder_transform.translation.x() + x_offset;
                let y = builder_transform.translation.y() + (i as f32) * y_width + y_offset;
                let z = 1.;

                // some temporary logic since bevy_rapier is slow to update from bevy
                // https://github.com/dimforge/bevy_rapier/issues/6
                let mut y_rot = 0.;
                if projectile.facing == CardinalDirection::West {
                    y_rot = std::f32::consts::PI;
                }
                let rotation = Quat::from_rotation_y(y_rot);
                let mut arrow_initial_transform = Transform::from_translation(Vec3::new(x, y, z));
                arrow_initial_transform.rotate(rotation);

                // println!("Spawning arrow at {:?}", arrow_initial_transform);

                let arrow_entity = commands
                    .spawn(SpriteSheetComponents {
                        texture_atlas: arrow_atlas_handle,
                        sprite: TextureAtlasSprite::new(0),
                        transform: arrow_initial_transform,
                        ..Default::default()
                    })
                    .with(projectile)
                    .with(projectile_timer)
                    .current_entity()
                    .unwrap();

                let arrow_body = RigidBodyBuilder::new_dynamic()
                    .translation(x, y, z)
                    .rotation(AngVector::new(0.0, y_rot, 0.0))
                    .lock_rotations()
                    .linvel(x_velocity, 0., 0.);
                let arrow_collider =
                    ColliderBuilder::cuboid(8., 4., 16.).user_data(arrow_entity.to_bits() as u128);

                commands.insert(arrow_entity, (arrow_body, arrow_collider));
            }
        }
    }
}
