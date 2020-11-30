use bevy::prelude::*;

use crate::asset_loader::AtlasHandles;
use crate::builder::{Builder, AnimationBundle};
use crate::world_map::{tile_to_position, Biome, WorldMap};
use bevy::render::camera::Camera;
use bevy_rapier3d::physics::RapierConfiguration;
use bevy_rapier3d::rapier::na::Vector;
use crate::data::animation::UnitType;
use crate::enemy::Enemy;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

pub const WORLD_MAP_RENDER_WIDTH: usize = 13;
pub const WORLD_MAP_RENDER_HEIGHT: usize = 10;
pub const ENEMY_DENSITY: f32 = 0.01;

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

fn generate_world(
    mut world_map: ResMut<WorldMap>,
    mut rapier_config: ResMut<RapierConfiguration>
) {
    for x in 200..300 {
        for y in 125..175 {
            let mut tile = world_map.get_tile_mut(x, y).unwrap();
            tile.biome = Biome::Desert;
            if (rand::random::<u32>() % 100) as f32 / 100. <= ENEMY_DENSITY {
                tile.contains_enemy = true;
                println!("Enemy should be spawned at {} {}", x, y)
            }
        }
    }
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
            let builder_atlas_handle = Handle::weak(atlas_handles.builder_id.unwrap());
            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: builder_atlas_handle,
                    sprite: TextureAtlasSprite::new(7),
                    transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                    ..Default::default()
                })
                .with_bundle(AnimationBundle::new(UnitType::Wizard))
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
                    let enemy_atlas_handle = Handle::weak(atlas_handles.enemy_id.unwrap());
                    let enemy_transform = tile_to_position(&center_tile, tile.x, tile.y);

                    let enemy_body = RigidBodyBuilder::new_dynamic()
                        .translation(
                            enemy_transform.translation.x(),
                            enemy_transform.translation.y(),
                            enemy_transform.translation.z()
                        );
                    let enemy_collider = ColliderBuilder::cuboid(16., 16., 16.);
                    commands
                        .spawn(SpriteSheetComponents {
                            texture_atlas: enemy_atlas_handle,
                            sprite: TextureAtlasSprite::new(7),
                            transform: enemy_transform,
                            ..Default::default()
                        })
                        .with(enemy_body)
                        .with(enemy_collider)
                        .with_bundle(AnimationBundle::new(UnitType::Enemy))
                        .with(Enemy::generic_enemy());
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
