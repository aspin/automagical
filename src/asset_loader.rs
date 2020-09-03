use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;
use bevy::asset::{HandleId, LoadState};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<MapSpriteHandles>()
            .init_resource::<AtlasHandles>()
            .add_startup_system(loader.system())
            .add_system(post_load.system());
    }
}

#[derive(Default)]
pub struct MapSpriteHandles {
    biome_handles: Vec<HandleId>,
    loaded: bool
}

#[derive(Default)]
pub struct AtlasHandles {
    pub grassland_biome_id: Option<HandleId>,
}

fn loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_sprite_handles: ResMut<MapSpriteHandles>,
) {
    map_sprite_handles.biome_handles = asset_server.load_asset_folder("assets/texture/biome").unwrap();

    let builder_handle = asset_server.load("assets/texture/builder.png").unwrap();

    commands
        .spawn(Camera2dComponents {
            orthographic_projection: OrthographicProjection {
                far: 10000.,
                ..Default::default()
            },
            scale: Scale(0.1),
            ..Default::default()
        })
        .spawn(SpriteComponents {
            material: materials.add(builder_handle.into()),
            translation: Translation::new(0.0, 0.0, 1.0),
            ..Default::default()
        });
}

fn post_load(
    asset_server: Res<AssetServer>,
    textures: Res<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut map_sprite_handles: ResMut<MapSpriteHandles>,
    mut atlas_handles: ResMut<AtlasHandles>,
) {
    if map_sprite_handles.loaded {
        return;
    }

    if let Some(LoadState::Loaded(_)) = asset_server.get_group_load_state(
        &map_sprite_handles.biome_handles
    ) {
        let grass_handle = asset_server
            .get_handle("assets/texture/biome/grass.png")
            .unwrap();
        let grass_texture = textures.get(&grass_handle).unwrap();
        let grassland_atlas = TextureAtlas::from_grid(
            grass_handle, grass_texture.size, 4, 1
        );

        let texture_atlas_handle = texture_atlases.add(grassland_atlas);

        atlas_handles.grassland_biome_id.replace(texture_atlas_handle.id);
        map_sprite_handles.loaded = true;
    }
}