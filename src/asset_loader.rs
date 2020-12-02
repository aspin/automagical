use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;

use crate::construction::CursorState;

pub const TILE_LENGTH: u32 = 16;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpriteHandles>()
            .init_resource::<AtlasHandles>()
            .add_startup_system(loader.system())
            .add_system(post_load.system());
    }
}

#[derive(Default)]
pub struct SpriteHandles {
    biome_handles: Vec<HandleUntyped>,
    builder_handle: Handle<Texture>,
    projectile_handles: Vec<HandleUntyped>,
    conveyor_handle: Handle<Texture>,
    enemy_handle: Handle<Texture>,
    loaded: bool,
}

#[derive(Default)]
pub struct AtlasHandles {
    pub grassland_biome_id: Option<HandleId>,
    pub desert_biome_id: Option<HandleId>,
    pub rocklands_biome_id: Option<HandleId>,
    pub builder_id: Option<HandleId>,
    pub arrow_id: Option<HandleId>,
    pub conveyor_id: Option<HandleId>,
    pub enemy_id: Option<HandleId>,
}

impl AtlasHandles {
    pub fn loaded(&self) -> bool {
        self.biomes_loaded()
            && self.builder_id.is_some()
            && self.projectiles_loaded()
            && self.conveyor_id.is_some()
            && self.enemy_id.is_some()
    }

    pub fn biomes_loaded(&self) -> bool {
        self.grassland_biome_id.is_some()
            && self.rocklands_biome_id.is_some()
            && self.desert_biome_id.is_some()
    }

    pub fn projectiles_loaded(&self) -> bool {
        self.arrow_id.is_some()
    }
}

fn loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_sprite_handles: ResMut<SpriteHandles>,
) {
    map_sprite_handles.biome_handles = asset_server.load_folder("texture/biome").unwrap();
    map_sprite_handles.projectile_handles = asset_server.load_folder("texture/projectile").unwrap();
    map_sprite_handles.builder_handle = asset_server.load("texture/wizard.png");
    map_sprite_handles.enemy_handle = asset_server.load("texture/enemy.png");
    map_sprite_handles.conveyor_handle = asset_server.load("texture/conveyor.png");

    let camera_entity = commands
        .spawn(Camera2dComponents {
            transform: Transform {
                scale: Vec3::new(0.3, 0.3, 0.3),
                translation: Vec3::new(0., 0., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .current_entity()
        .unwrap();

    commands.insert_resource(CursorState {
        camera_entity,
        cursor: Default::default(),
        cursor_position: Option::None,
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

    println!("Loading assets...");

    let biomes_loaded = atlas_handles.biomes_loaded();
    let tile_size = Vec2::new(TILE_LENGTH as f32, TILE_LENGTH as f32);
    if !biomes_loaded {
        if let LoadState::Loaded = asset_server
            .get_group_load_state(sprite_handles.biome_handles.iter().map(|handle| handle.id))
        {
            let grass_handle = asset_server.get_handle("texture/biome/grass.png");
            let grassland_atlas = TextureAtlas::from_grid(grass_handle, tile_size, 4, 1);

            let grassland_atlas_handle = texture_atlases.add(grassland_atlas);

            atlas_handles
                .grassland_biome_id
                .replace(grassland_atlas_handle.id);

            let desert_handle = asset_server.get_handle("texture/biome/desert.png");
            let desert_atlas = TextureAtlas::from_grid(desert_handle, tile_size, 4, 1);

            let desert_atlas_handle = texture_atlases.add(desert_atlas);
            atlas_handles
                .desert_biome_id
                .replace(desert_atlas_handle.id);

            let rockland_handle = asset_server.get_handle("texture/biome/rocklands.png");
            let rockland_atlas = TextureAtlas::from_grid(rockland_handle, tile_size, 4, 1);

            let rockland_atlas_handle = texture_atlases.add(rockland_atlas);
            atlas_handles
                .rocklands_biome_id
                .replace(rockland_atlas_handle.id);
        }
    }

    let projectile_loaded = atlas_handles.projectiles_loaded();
    if !projectile_loaded {
        if let LoadState::Loaded = asset_server.get_group_load_state(
            sprite_handles
                .projectile_handles
                .iter()
                .map(|handle| handle.id),
        ) {
            let arrow_handle = asset_server.get_handle("texture/projectile/arrow.png");
            let arrow_texture = textures.get(&arrow_handle).unwrap();
            let arrow_atlas = TextureAtlas::from_grid(arrow_handle, arrow_texture.size, 1, 1);

            let arrow_atlas_handle = texture_atlases.add(arrow_atlas);
            atlas_handles.arrow_id.replace(arrow_atlas_handle.id);
        }
    }

    let builder_loaded = atlas_handles.builder_id.is_some();
    if !builder_loaded {
        let builder_handle = asset_server.get_handle(&sprite_handles.builder_handle);
        if let LoadState::Loaded = asset_server.get_load_state(&builder_handle) {
            let builder_atlas = TextureAtlas::from_grid(builder_handle, tile_size, 7, 3);
            let builder_atlas_handle = texture_atlases.add(builder_atlas);
            atlas_handles.builder_id.replace(builder_atlas_handle.id);
        }
    }

    let enemy_loaded = atlas_handles.enemy_id.is_some();
    if !enemy_loaded {
        let enemy_handle = asset_server.get_handle(&sprite_handles.enemy_handle);
        if let LoadState::Loaded = asset_server.get_load_state(&enemy_handle) {
            let enemy_atlas = TextureAtlas::from_grid(enemy_handle, tile_size, 7, 3);
            let enemy_atlas_handle = texture_atlases.add(enemy_atlas);
            atlas_handles.enemy_id.replace(enemy_atlas_handle.id);
        }
    }

    let conveyor_loaded = atlas_handles.conveyor_id.is_some();
    if !conveyor_loaded {
        let conveyor_handle = asset_server.get_handle(&sprite_handles.conveyor_handle);
        if let LoadState::Loaded = asset_server.get_load_state(&conveyor_handle) {
            let conveyor_atlas = TextureAtlas::from_grid(conveyor_handle, tile_size, 1, 1);
            let conveyor_atlas_handle = texture_atlases.add(conveyor_atlas);
            atlas_handles.conveyor_id.replace(conveyor_atlas_handle.id);
        }
    }

    if atlas_handles.loaded() {
        sprite_handles.loaded = true;
    }
}
