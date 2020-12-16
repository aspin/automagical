use crate::animation::{Animated, AnimationState, UnitType, AnimationBundle};
use crate::asset_loader::AtlasHandles;
use crate::data;
use crate::data::AssetType;
use crate::projectile::Projectile;
use crate::weapon::Weapon;
use bevy::prelude::*;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier3d::rapier::math::{AngVector, Rotation};
use bevy_rapier3d::rapier::na::{Isometry3, Vector3};

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(initialize_player.system())
            .add_system(produce_projectiles.system());
    }
}

pub struct Player {
    pub builder_entity: Entity,
}

impl Player {
    pub fn new(builder_entity: Entity) -> Self {
        Player { builder_entity }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum BuilderMode {
    Construct,
    Combat,
}

pub struct Builder {
    pub name: String,
    pub mode: BuilderMode,

    // last aimed cursor location
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

    /// Returns a vector from the body to the builder's aimed location.
    /// Expected usage is for aiming from the builder's body component.
    pub fn to_aimed_location(&self, position: &Isometry3<f32>) -> Option<Vector3<f32>> {
        if let Some(aim_location) = self.aim_location {
            let aimed_vector =
                Vector3::new(aim_location.x(), aim_location.y(), position.translation.z);
            Some((aimed_vector - &position.translation.vector).normalize())
        } else {
            Option::None
        }
    }
}

pub fn initialize_player(
    mut commands: Commands
) {
    let builder_entity = commands
        .spawn((Builder::new("Bob the builder"), Weapon::magic_bow()))
        .with_bundle(AnimationBundle::new(UnitType::Wizard))
        .current_entity()
        .unwrap();
    commands.insert_resource(Player::new(builder_entity));
}

pub fn produce_projectiles(
    mut commands: Commands,
    atlas_handles: Res<AtlasHandles>,
    rigid_body_set: Res<RigidBodySet>,
    animated: &Animated,
    builder_body_handle: &RigidBodyHandleComponent,
    builder: &Builder,
    weapon: &Weapon,
) {
    if let Some(arrow_id) = atlas_handles.get_asset(AssetType::Arrow) {
        if animated.state == AnimationState::Attack && animated.animation_index == 1 {
            let builder_body = rigid_body_set.get(builder_body_handle.handle()).unwrap();

            let projectile = Projectile::arrow();
            let projectile_positions =
                compute_projectile_positions(builder_body.position(), builder, weapon, &projectile);

            for (transform, body) in projectile_positions {
                let arrow_atlas_handle = Handle::weak(arrow_id);
                let projectile_timer = Timer::from_seconds((&projectile).ttl, false);

                let arrow_entity = commands
                    .spawn(SpriteSheetComponents {
                        texture_atlas: arrow_atlas_handle,
                        sprite: TextureAtlasSprite::new(0),
                        transform,
                        ..Default::default()
                    })
                    .with(projectile.clone())
                    .with(projectile_timer)
                    .current_entity()
                    .unwrap();

                let arrow_collider = data::get_collision_data(UnitType::Arrow)
                    .user_data(arrow_entity.to_bits() as u128);

                commands.insert(arrow_entity, (body, arrow_collider));
            }
        }
    }
}

fn compute_projectile_positions(
    builder_position: &Isometry3<f32>,
    builder: &Builder,
    weapon: &Weapon,
    projectile: &Projectile,
) -> Vec<(Transform, RigidBodyBuilder)> {
    let mut result = Vec::with_capacity(weapon.spread_range().len());
    for i in weapon.spread_range() {
        let mut arrow_position = (*builder_position).clone();

        let aim_vector = builder.to_aimed_location(builder_position).unwrap();
        let velocity_vector =
            aim_vector.clone() * (projectile.speed + weapon.projectile_launch_speed);
        let normal_vector = Vector3::new(-aim_vector.y, aim_vector.x, aim_vector.z);
        let z_plane_zeroes = Vector3::new(1., 0., aim_vector.z);

        let mut z_rot = velocity_vector.angle(&z_plane_zeroes);
        if aim_vector.y < 0. {
            z_rot *= -1.;
        }

        arrow_position.translation.vector += aim_vector * weapon.size;
        arrow_position.translation.vector += normal_vector * (i as f32) * weapon.projectile_spread;
        arrow_position.rotation = Rotation::new(AngVector::new(0., 0., z_rot));

        // some temporary logic since bevy_rapier is slow to update from bevy
        // https://github.com/dimforge/bevy_rapier/issues/6
        let transform_rotation = Quat::from_rotation_z(z_rot);
        let mut arrow_initial_transform = Transform::from_translation(Vec3::new(
            arrow_position.translation.x,
            arrow_position.translation.y,
            arrow_position.translation.z,
        ));
        arrow_initial_transform.rotate(transform_rotation);

        let arrow_body = RigidBodyBuilder::new_dynamic()
            .position(arrow_position)
            .lock_rotations()
            .linvel(velocity_vector.x, velocity_vector.y, velocity_vector.z);

        result.push((arrow_initial_transform, arrow_body))
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projectile::ProjectileType;

    #[test]
    fn test_to_aimed_location() {
        let position = Isometry3::new(Vector3::zeros(), Vector3::zeros());
        let mut builder = Builder::new("tester");

        builder.aim_location.replace(Vec4::new(1., 0., 0., 0.));
        assert_eq!(
            Vector3::new(1., 0., 0.),
            builder.to_aimed_location(&position).unwrap()
        );

        builder.aim_location.replace(Vec4::new(0., 1., 0., 0.));
        assert_eq!(
            Vector3::new(0., 1., 0.),
            builder.to_aimed_location(&position).unwrap()
        );

        builder.aim_location.replace(Vec4::new(10., 0., 0., 0.));
        assert_eq!(
            Vector3::new(1., 0., 0.),
            builder.to_aimed_location(&position).unwrap()
        );

        builder.aim_location.replace(Vec4::new(10., 10., 0., 0.));
        assert_eq!(
            Vector3::new(1., 1., 0.).normalize(),
            builder.to_aimed_location(&position).unwrap()
        );
    }

    fn assert_projectile_position(
        transform: &Transform,
        body_builder: &RigidBodyBuilder,
        x: f32,
        y: f32,
        z: f32,
        rotation_angle: f32,
        velocity_x: f32,
        velocity_y: f32,
        velocity_z: f32,
    ) {
        let body = body_builder.build();

        // check translations
        assert_eq!(x, transform.translation.x());
        assert_eq!(y, transform.translation.y());
        assert_eq!(z, transform.translation.z());
        assert_eq!(x, body.position().translation.x);
        assert_eq!(y, body.position().translation.y);
        assert_eq!(z, body.position().translation.z);

        // check rotations
        assert_eq!(Quat::from_rotation_z(rotation_angle), transform.rotation);
        assert_eq!(
            Rotation::new(AngVector::new(0., 0., rotation_angle)),
            body.position().rotation
        );

        // check velocity
        assert_eq!(velocity_x, body.linvel().x);
        assert_eq!(velocity_y, body.linvel().y);
        assert_eq!(velocity_z, body.linvel().z);
    }

    #[test]
    fn test_compute_projectile_positions() {
        let mut builder = Builder::new("tester");
        let builder_position = Isometry3::new(Vector3::zeros(), Vector3::zeros());
        let weapon = Weapon::new(10., Some(ProjectileType::Arrow), 5, 100., 10.);
        let projectile = Projectile::new(10, 10., true, 500.);

        builder.aim_location.replace(Vec4::new(1., 0., 0., 0.));
        let projectile_positions =
            compute_projectile_positions(&builder_position, &builder, &weapon, &projectile);

        assert_eq!(5, projectile_positions.len());

        let (first_transform, first_body_builder) = &projectile_positions[0];
        assert_projectile_position(
            first_transform,
            first_body_builder,
            10.,
            -20.,
            0.,
            0.,
            600.,
            0.,
            0.,
        );

        let (third_transform, third_body_builder) = &projectile_positions[2];
        assert_projectile_position(
            third_transform,
            third_body_builder,
            10.,
            0.,
            0.,
            0.,
            600.,
            0.,
            0.,
        );
    }
}
