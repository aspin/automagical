use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;
use bevy::asset::{HandleId, LoadState};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<SpriteHandles>()
            .init_resource::<AtlasHandles>()
            .add_startup_system(loader.system())
            .add_system(post_load.system());
    }
}

#[derive(Default)]
pub struct SpriteHandles {
    biome_handles: Vec<HandleId>,
    builder_handle: Handle<Texture>,
    loaded: bool
}

#[derive(Default)]
pub struct AtlasHandles {
    pub grassland_biome_id: Option<HandleId>,
    pub builder_id: Option<HandleId>,
}

impl AtlasHandles {
    pub fn loaded(&self) -> bool {
        self.grassland_biome_id.is_some() && self.builder_id.is_some()
    }
}

fn loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_sprite_handles: ResMut<SpriteHandles>,
) {
    map_sprite_handles.biome_handles = asset_server.load_asset_folder("assets/texture/biome").unwrap();
    map_sprite_handles.builder_handle = asset_server.load("assets/texture/wizard.png").unwrap();

    commands
        .spawn(Camera2dComponents {
            orthographic_projection: OrthographicProjection {
                far: 10000.,
                ..Default::default()
            },
            scale: Scale(0.3),
            ..Default::default()
        });
}

fn post_load(
    asset_server: Res<AssetServer>,
    textures: Res<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut atlas_handles: ResMut<AtlasHandles>,
) {
    if sprite_handles.loaded {
        return;
    }

    let grassland_loaded = atlas_handles.grassland_biome_id.is_some();
    if !grassland_loaded {
        if let Some(LoadState::Loaded(_)) = asset_server.get_group_load_state(
            &sprite_handles.biome_handles
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
        }
    }

    let builder_loaded = atlas_handles.builder_id.is_some();
    if !builder_loaded {
        let builder_handle = sprite_handles.builder_handle;
        if let Some(LoadState::Loaded(_)) = asset_server.get_load_state(builder_handle) {
            let builder_texture = textures.get(&builder_handle).unwrap();
            let builder_atlas = TextureAtlas::from_grid(
                builder_handle, builder_texture.size, 7, 3
            );
            let builder_atlas_handle = texture_atlases.add(builder_atlas);
            atlas_handles.builder_id.replace(builder_atlas_handle.id);
        }
    }

    if builder_loaded && grassland_loaded {
        sprite_handles.loaded = true;
    }
}