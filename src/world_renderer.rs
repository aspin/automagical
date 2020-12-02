use bevy::prelude::*;

use crate::asset_loader::AtlasHandles;
use crate::builder::{AnimationBundle, Builder};
use crate::animation::UnitType;
use crate::enemy::Enemy;
use crate::world_map::{tile_to_position, WorldMap};
use bevy::render::camera::Camera;
use bevy_rapier3d::physics::RapierConfiguration;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::rapier::na::Vector;
use crate::biome::Biome;
use crate::data::AssetType;

pub const WORLD_MAP_RENDER_WIDTH: usize = 13;
pub const WORLD_MAP_RENDER_HEIGHT: usize = 10;
pub const ENEMY_DENSITY: f32 = 0.001;

pub struct MapGeneratorPlugin;

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<World>()
            .add_startup_system(generate_world.system())
            .add_system(render_world.system());
    }
}

#[derive(Default)]
pub struct World {
    generated: bool,
}

fn generate_world(mut world_map: ResMut<WorldMap>, mut rapier_config: ResMut<RapierConfiguration>) {
    for x in 200..300 {
        for y in 125..175 {
            let mut tile = world_map.get_tile_mut(x, y).unwrap();
            tile.biome = Biome::Desert;
            if (rand::random::<u32>() % 1000) as f32 / 1000. <= ENEMY_DENSITY {
                tile.contains_enemy = true;
                // println!("Enemy should be spawned at {} {}", x, y)
            }
        }
    }
    world_map.get_tile_mut(160, 150).unwrap().contains_enemy = true;
    rapier_config.gravity = Vector::y() * 0.;
}

fn render_world(
    mut commands: Commands,
    atlas_handles: Res<AtlasHandles>,
    mut world: ResMut<World>,
    mut world_map: ResMut<WorldMap>,
    query_camera: Query<(&Camera, &Transform)>,
) {
    if atlas_handles.loaded() {
        if !world.generated {
            let builder_atlas_handle = Handle::weak(atlas_handles.get_asset(AssetType::Builder).unwrap());
            let builder_x = 0.;
            let builder_y = 0.;
            let builder_z = 1.;

            let builder_body = RigidBodyBuilder::new_dynamic()
                .translation(builder_x, builder_y, builder_z)
                .lock_rotations()
                .lock_translations();
            let builder_collider = ColliderBuilder::cuboid(16., 16., 16.);
            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: builder_atlas_handle,
                    sprite: TextureAtlasSprite::new(7),
                    transform: Transform::from_translation(Vec3::new(
                        builder_x, builder_y, builder_z,
                    )),
                    ..Default::default()
                })
                .with_bundle(AnimationBundle::new(UnitType::Wizard))
                .with(builder_body)
                .with(builder_collider)
                .with(Builder::new("Bob the builder"));

            world.generated = true;
        }

        // can probably be assertion?
        let query_camera_iterator = &mut query_camera.iter();
        if let Some((_camera, camera_transform)) = query_camera_iterator.into_iter().next() {
            let center_tile = world_map.center_tile();
            let (tiles_to_render, tiles_to_despawn) = world_map.get_tiles_for_update(
                camera_transform.translation.x(),
                camera_transform.translation.y(),
            );
            for tile in tiles_to_render {
                // println!("render {} {} as {:?}", tile.x, tile.y, tile.biome);
                if tile.rendered_entity.is_none() {
                    tile.rendered_entity.replace(
                        commands
                            .spawn(SpriteSheetComponents {
                                texture_atlas: tile.get_biome_handle(&atlas_handles),
                                sprite: TextureAtlasSprite::new(rand::random::<u32>() % 4),
                                transform: tile_to_position(&center_tile, tile.x, tile.y),
                                ..Default::default()
                            })
                            .current_entity()
                            .unwrap(),
                    );
                }
                if tile.contains_enemy {
                    let enemy_atlas_handle = Handle::weak(atlas_handles.get_asset(AssetType::Enemy).unwrap());
                    let enemy_transform = tile_to_position(&center_tile, tile.x, tile.y);
                    let enemy_entity = commands
                        .spawn(SpriteSheetComponents {
                            texture_atlas: enemy_atlas_handle,
                            sprite: TextureAtlasSprite::new(7),
                            transform: enemy_transform,
                            ..Default::default()
                        })
                        .with_bundle(AnimationBundle::new(UnitType::Enemy))
                        .with(Enemy::generic_enemy())
                        .current_entity()
                        .unwrap();

                    let enemy_body = RigidBodyBuilder::new_dynamic()
                        .translation(
                            enemy_transform.translation.x(),
                            enemy_transform.translation.y(),
                            enemy_transform.translation.z(),
                        )
                        .lock_rotations()
                        .mass(1000., false);
                    let enemy_collider = ColliderBuilder::cuboid(16., 16., 16.)
                        .user_data(enemy_entity.to_bits() as u128);
                    commands.insert(enemy_entity, (enemy_body, enemy_collider));
                }
            }
            for tile in tiles_to_despawn {
                let entity = tile.rendered_entity.unwrap();
                commands.despawn(entity);
                tile.rendered_entity.take();
            }
        }
    }
}
