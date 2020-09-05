use bevy::prelude::*;

use crate::asset_loader::AtlasHandles;

pub struct MapGeneratorPlugin;

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<World>()
            .add_system(generate_world.system());
    }
}

#[derive(Default)]
pub struct World {
    generated: bool
}

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
    fn new(name: &str) -> Builder {
        Builder {
            name: String::from(name),
            state: BuilderState::Idle,
            animation_index: 0,
        }
    }
}

fn generate_world(
    mut commands: Commands,
    atlas_handles: Res<AtlasHandles>,
    mut world: ResMut<World>
) {
    if world.generated {
        return;
    }

    if atlas_handles.loaded() {
        let grassland_atlas_handle = Handle::from_id(atlas_handles.grassland_biome_id.unwrap());
        let desert_atlas_handle = Handle::from_id(atlas_handles.desert_biome_id.unwrap());

        for x in -10..10 {
            for y in -10..10 {
                commands.spawn(SpriteSheetComponents {
                    texture_atlas: grassland_atlas_handle,
                    sprite: TextureAtlasSprite::new(rand::random::<u32>() % 4),
                    translation: Translation::new((x * 16) as f32, (y * 16) as f32, 0.0),
                    ..Default::default()
                });
            }
        }

        for x in -5..-2 {
            for y in -9..1 {
                commands.spawn(SpriteSheetComponents {
                    texture_atlas: desert_atlas_handle,
                    sprite: TextureAtlasSprite::new(rand::random::<u32>() % 4),
                    translation: Translation::new((x * 16) as f32, (y * 16) as f32, 0.1),
                    ..Default::default()
                });
            }
        }


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
}