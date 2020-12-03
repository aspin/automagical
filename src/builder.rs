use crate::asset_loader::AtlasHandles;
use crate::projectile::{Projectile, ARROW_SPEED};
use bevy::prelude::*;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::rapier::math::AngVector;
use crate::animation::{AnimationState, Animated, CardinalDirection};
use crate::data::AssetType;

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

pub fn produce_projectiles(
    mut commands: Commands,
    atlas_handles: Res<AtlasHandles>,
    animated: &Animated,
    builder_transform: &Transform,
    _builder: &Builder,
) {
    if let Some(arrow_id) = atlas_handles.get_asset(AssetType::Arrow) {
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
