use bevy::prelude::*;

use crate::asset_loader::AtlasHandles;
use crate::builder::Builder;
use crate::world_map::{WorldMap, tile_to_position, Biome};
use bevy::render::camera::Camera;
use bevy_rapier3d::physics::Gravity;
use bevy_rapier3d::rapier::na::Vector;

pub const WORLD_MAP_RENDER_WIDTH: usize = 13;
pub const WORLD_MAP_RENDER_HEIGHT: usize = 10;
pub const TILE_LENGTH: u32 = 16;

pub struct MapGeneratorPlugin;

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<World>()
            .add_startup_system(generate_world.system())
            .add_system(render_world.system());
    }
}

#[derive(Default)]
pub struct World {
    generated: bool
}

fn generate_world(
    mut world_map: ResMut<WorldMap>,
    mut gravity: ResMut<Gravity>,
) {
    for x in 200..300 {
        for y in 125..175 {
            world_map.get_tile_mut(x, y).unwrap().biome = Biome::Desert;
        }
    }
    gravity.0 = Vector::y() * 0.;
}

fn render_world(
    mut commands: Commands,
    atlas_handles: Res<AtlasHandles>,
    mut world: ResMut<World>,
    mut world_map: ResMut<WorldMap>,
    mut query_camera: Query<(&Camera, &Translation)>,
) {
    if atlas_handles.loaded() {
        if !world.generated {
            let builder_atlas_handle = Handle::from_id(atlas_handles.builder_id.unwrap());
            commands
                .spawn(
                    SpriteSheetComponents {
                        texture_atlas: builder_atlas_handle,
                        sprite: TextureAtlasSprite::new(7),
                        translation: Translation::new(0., 0., 1.),
                        ..Default::default()
                    }
                )
                .with(Timer::from_seconds(0.5, false))
                .with(Builder::new("Bob the builder"));

            world.generated = true;
        }

        // can probably be assertion?
        let query_camera_iterator = &mut query_camera.iter();
        if let Some((_camera, camera_translation)) = query_camera_iterator.into_iter().next() {
            let center_tile = world_map.center_tile();
            let (tiles_to_render, tiles_to_despawn) = world_map.get_tiles_for_update(
                camera_translation.x(), camera_translation.y()
            );
            for tile in tiles_to_render {
                // println!("render {} {} as {:?}", tile.x, tile.y, tile.biome);
                if tile.rendered_entity.is_none() {
                    tile.rendered_entity.replace(
                        commands
                            .spawn(SpriteSheetComponents {
                                texture_atlas: tile.get_biome_handle(&atlas_handles),
                                sprite: TextureAtlasSprite::new(rand::random::<u32>() % 4),
                                translation: tile_to_position(&center_tile, tile.x, tile.y),
                                ..Default::default()
                            })
                            .current_entity()
                            .unwrap()
                    );
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
