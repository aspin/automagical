use crate::asset_loader::AtlasHandles;
use crate::projectile::Projectile;
use bevy::prelude::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

const ANIMATION_SPEED: f32 = 0.5;

pub struct Builder {
    pub name: String,
    pub state: BuilderState,
    pub animation_index: u32,
}

#[derive(PartialEq)]
pub enum BuilderState {
    Idle,
    Move,
    Attack,
}

impl Builder {
    pub fn new(name: &str) -> Builder {
        Builder {
            name: String::from(name),
            state: BuilderState::Idle,
            animation_index: 0,
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

                let y_offset = -4.;
                let y_width = 4.;
                let arrow_body = RigidBodyBuilder::new_dynamic()
                    .translation(
                        builder_transform.translation.x() + 16.,
                        builder_transform.translation.y() + (i as f32) * y_width + y_offset,
                        2.,
                    )
                    .linvel(1000., 0., 0.);
                let arrow_collider = ColliderBuilder::cuboid(0., 0., 0.);
                let projectile = Projectile::arrow();
                let projectile_timer = Timer::from_seconds(projectile.ttl, false);

                // println!("Spawning arrow at {} {}", builder_transform.translation.x(), builder_transform.translation.y());

                commands
                    .spawn(SpriteSheetComponents {
                        texture_atlas: arrow_atlas_handle,
                        sprite: TextureAtlasSprite::new(0),
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
