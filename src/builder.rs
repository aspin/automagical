use crate::animation::{Animated, AnimationState, UnitType};
use crate::asset_loader::AtlasHandles;
use crate::data::AssetType;
use crate::projectile::{Projectile, ARROW_SPEED, ARROW_OFFSET};
use bevy::prelude::*;
use bevy_rapier3d::rapier::dynamics::{RigidBodyBuilder, RigidBody, RigidBodySet};
use bevy_rapier3d::rapier::math::{AngVector, Rotation};
use bevy_rapier3d::rapier::na::Vector3;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use crate::data;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum BuilderMode {
    Construct,
    Combat
}

pub struct Builder {
    pub name: String,
    pub mode: BuilderMode,
    pub aim_location: Option<Vec4>,
}

impl Builder {
    pub fn new(name: &str) -> Self {
        Builder {
            name: String::from(name),
            mode: BuilderMode::Combat,
            aim_location: Option::None,
        }
    }

    pub fn toggle_mode(&mut self) {
        if self.mode == BuilderMode::Construct {
            self.mode = BuilderMode::Combat
        } else {
            self.mode = BuilderMode::Construct
        }
    }

    pub fn position_to_aimed_location(&self, rigid_body: &RigidBody) -> Option<Vector3<f32>> {
        if let Some(aim_location) = self.aim_location {
            let aimed_vector = Vector3::new(
                aim_location.x(), aim_location.y(), rigid_body.position().translation.z
            );
            Some((aimed_vector - rigid_body.position().translation.vector).normalize())
        } else {
            Option::None
        }
    }
}

pub fn produce_projectiles(
    mut commands: Commands,
    atlas_handles: Res<AtlasHandles>,
    rigid_body_set: Res<RigidBodySet>,
    animated: &Animated,
    builder_body_handle: &RigidBodyHandleComponent,
    builder: &Builder,
) {
    if let Some(arrow_id) = atlas_handles.get_asset(AssetType::Arrow) {
        if animated.state == AnimationState::Attack && animated.animation_index == 3 {
            let builder_body = rigid_body_set.get(builder_body_handle.handle()).unwrap();
            // for i in 0..3 {

            let arrow_atlas_handle = Handle::weak(arrow_id);
            let projectile = Projectile::arrow();
            let projectile_timer = Timer::from_seconds(projectile.ttl, false);

            let aim_vector = builder.position_to_aimed_location(builder_body).unwrap();
            let velocity_vector = aim_vector.clone() * ARROW_SPEED;
            let z_plane_zeroes = Vector3::new(1., 0., aim_vector.z);

            let mut y_rot = velocity_vector.angle(&z_plane_zeroes);
            if aim_vector.y < 0. {
                y_rot *= -1.;
            }

            // println!(
            //     "firing arrows at {:?}. arrow vector: {:?} arrow rotation {:?}",
            //     builder.aim_location,
            //     aim_vector,
            //     y_rot
            // );

            let mut arrow_position = builder_body.position().clone();
            arrow_position.translation.vector += aim_vector * ARROW_OFFSET;
            arrow_position.rotation = Rotation::new(
                AngVector::new(0., 0., y_rot)
            );

            // some temporary logic since bevy_rapier is slow to update from bevy
            // https://github.com/dimforge/bevy_rapier/issues/6
            let transform_rotation = Quat::from_rotation_z(y_rot);
            let mut arrow_initial_transform = Transform::from_translation(
                Vec3::new(
                    arrow_position.translation.x,
                    arrow_position.translation.y,
                    arrow_position.translation.z
                )
            );
            arrow_initial_transform.rotate(transform_rotation);

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
                .position(arrow_position)
                .lock_rotations()
                .linvel(velocity_vector.x, velocity_vector.y, velocity_vector.z);
            let arrow_collider = data::get_collision_data(UnitType::Arrow)
                .user_data(arrow_entity.to_bits() as u128);

            commands.insert(arrow_entity, (arrow_body, arrow_collider));
            // }
        }
    }
}
