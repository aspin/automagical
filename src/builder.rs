use crate::asset_loader::AtlasHandles;
use crate::projectile::{Projectile, ARROW_SPEED};
use bevy::prelude::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::rapier::math::AngVector;

const ANIMATION_SPEED: f32 = 0.5;

pub struct Builder {
    pub name: String,
    pub state: BuilderState,
    pub animation_index: u32,
    pub facing: CardinalDirection,
}

#[derive(PartialEq)]
pub enum BuilderState {
    Idle,
    Move,
    Attack,
}

#[derive(PartialEq, Copy, Clone)]
pub enum CardinalDirection {
    North,
    South,
    West,
    East
}

impl Builder {
    pub fn new(name: &str) -> Builder {
        Builder {
            name: String::from(name),
            state: BuilderState::Idle,
            animation_index: 0,
            facing: CardinalDirection::East,
        }
    }
}

pub fn animate(mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Builder)>) {
    for (mut timer, mut sprite, mut builder) in query.iter_mut() {
        if timer.finished {
            let (mut offset, mut duration, length, loop_around) = match builder.state {
                BuilderState::Idle => (7, vec![0.5, 0.1, 0.06, 0.1, 0.1, 0.1], 6, true),
                BuilderState::Attack => (0, vec![0.5, 0.1, 0.3, 0.1, 0.1, 0.1, 0.1], 7, false),
                BuilderState::Move => (14, vec![0.5, 0.1, 0.1, 0.1], 4, false),
            };

            let next_index = builder.animation_index + 1;
            if next_index >= length {
                if loop_around {
                    builder.animation_index = next_index % length
                } else {
                    builder.animation_index = 0;
                    builder.state = BuilderState::Idle;
                    offset = 7;
                    duration = vec![0.5, 0.1, 0.06, 0.1, 0.1, 0.1];
                }
            } else {
                builder.animation_index = next_index;
            }

            sprite.index = offset + builder.animation_index;

            timer.reset();
            timer.duration = duration[builder.animation_index as usize] * ANIMATION_SPEED;
        }
    }
}

pub fn produce_projectiles(
    mut commands: Commands,
    atlas_handles: Res<AtlasHandles>,
    builder: &Builder,
    builder_transform: &Transform,
) {
    if let Some(arrow_id) = atlas_handles.arrow_id {
        if builder.state == BuilderState::Attack && builder.animation_index == 3 {
            for i in 0..3 {
                let arrow_atlas_handle = Handle::weak(arrow_id);
                let projectile = Projectile::arrow(builder.facing.clone());
                let projectile_timer = Timer::from_seconds(projectile.ttl, false);

                let mut x_offset: f32 = 16.;
                let mut x_velocity: f32 = ARROW_SPEED;
                if builder.facing == CardinalDirection::West {
                    x_offset *= -1.;
                    x_velocity *= -1.;
                }
                let y_offset = -4.;
                let y_width = 4.;

                let x = builder_transform.translation.x() + x_offset;
                let y = builder_transform.translation.y() + (i as f32) * y_width + y_offset;
                let z = 2.;

                // some temporary logic since bevy_rapier is slow to update from bevy
                // https://github.com/dimforge/bevy_rapier/issues/6
                let mut y_rot = 0.;
                if projectile.facing == CardinalDirection::West {
                    y_rot = std::f32::consts::PI;
                }
                let rotation = Quat::from_rotation_y(std::f32::consts::PI);
                let mut arrow_initial_transform = Transform::from_translation(
                    Vec3::new(x, y, z)
                );
                arrow_initial_transform.rotate(rotation);

                let arrow_body = RigidBodyBuilder::new_dynamic()
                    .translation(x, y, z)
                    .rotation(AngVector::new(0.0, y_rot, 0.0))
                    .linvel(x_velocity, 0., 0.);
                let arrow_collider = ColliderBuilder::cuboid(0., 0., 0.);

                // println!("Spawning arrow at {:?}", arrow_initial_transform);

                commands
                    .spawn(SpriteSheetComponents {
                        texture_atlas: arrow_atlas_handle,
                        sprite: TextureAtlasSprite::new(0),
                        transform: arrow_initial_transform,
                        ..Default::default()
                    })
                    .with(projectile)
                    .with(arrow_body)
                    .with(arrow_collider)
                    .with(projectile_timer);
            }
        }
    }
}
