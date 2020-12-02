use std::collections::HashMap;

use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;

use crate::construction::CursorState;
use crate::data;
use crate::biome::Biome;
use crate::data::{AssetType, AssetInfo, AssetGroup};

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
    asset_group_handles: HashMap<AssetGroup, Vec<HandleUntyped>>,
    loaded: bool,
}

impl SpriteHandles {
    fn add_asset_group(&mut self, asset_group: AssetGroup, asset_server: &AssetServer) {
        let asset_group_info = data::get_asset_group_info(asset_group);
        let asset_handles = asset_server.load_folder(asset_group_info.folder_path.as_str()).unwrap();
        self.asset_group_handles.insert(asset_group, asset_handles);
    }

    fn get_asset_group(&self, asset_group: AssetGroup) -> Option<&Vec<HandleUntyped>> {
        self.asset_group_handles.get(&asset_group)
    }
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

    pub handles: HashMap<AssetType, HandleId>,
    pub handle_groups: HashMap<AssetGroup, Vec<HandleId>>,
}

impl AtlasHandles {
    fn try_load_handle_group(
        &mut self,
        asset_group: AssetGroup,
        sprite_handles: &ResMut<SpriteHandles>,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>
    ) {
        if !self.handle_groups.contains_key(&asset_group) {
            if let Some(asset_group_sprites) = sprite_handles.get_asset_group(asset_group) {
                if are_assets_loaded(asset_group_sprites, &asset_server) {
                    let asset_group_info = data::get_asset_group_info(asset_group);
                    let mut asset_group_handles = Vec::with_capacity(
                        asset_group_info.assets_info.len()
                    );
                    for (asset_type, _asset_info) in asset_group_info.assets_info {
                        let handle_id = self.load_handle(asset_type, asset_server, texture_atlases);
                        // TODO: use lifetime + reference instead
                        asset_group_handles.push(handle_id.clone())
                    }
                    self.handle_groups.insert(asset_group, asset_group_handles);
                }
            }
        }
    }

    fn load_handle(
        &mut self,
        asset_type: AssetType,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>
    ) -> &HandleId {
        let asset_info = data::get_asset_info(asset_type);
        self.handles.insert(
            asset_type.clone(),
            load_asset_atlas_of_group(asset_info, asset_server, texture_atlases)
        );
        self.handles.get(&asset_type).unwrap()
    }

    pub fn get_asset(&self, asset_type: AssetType) -> Option<HandleId> {
        if let Some(handle_id) = self.handles.get(&asset_type) {
            Option::Some(handle_id.clone())
        } else {
            Option::None
        }
    }

    fn assets_loaded(&self, asset_types: Vec<AssetType>) -> bool {
        for asset_type in asset_types {
            if !self.handles.contains_key(&asset_type) {
                return false;
            }
        }
        true
    }

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
        self.assets_loaded(vec![AssetType::Arrow])
    }
}

fn loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_sprite_handles: ResMut<SpriteHandles>,
) {
    map_sprite_handles.biome_handles = asset_server.load_folder("texture/biome").unwrap();
    map_sprite_handles.projectile_handles = asset_server.load_folder(
        data::get_asset_group_info(AssetGroup::Projectile).folder_path.as_str()
    ).unwrap();
    map_sprite_handles.builder_handle = asset_server.load(data::get_asset_sprite_path(AssetType::Builder).as_str());
    map_sprite_handles.enemy_handle = asset_server.load(data::get_asset_sprite_path(AssetType::Enemy).as_str());
    map_sprite_handles.conveyor_handle = asset_server.load(data::get_asset_sprite_path(AssetType::Conveyor).as_str());

    map_sprite_handles.add_asset_group(AssetGroup::Projectile, &asset_server);

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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut atlas_handles: ResMut<AtlasHandles>,
) {
    if sprite_handles.loaded {
        return;
    }

    println!("Loading assets...");

    load_biome_atlases(&mut atlas_handles, &asset_server, &sprite_handles, &mut texture_atlases);

    atlas_handles.try_load_handle_group(
        AssetGroup::Projectile, &sprite_handles, &asset_server, &mut texture_atlases
    );

    maybe_load_asset_atlas(
        &mut atlas_handles.builder_id,
        &sprite_handles.builder_handle,
        &asset_server,
        &mut texture_atlases,
        data::get_asset_info(AssetType::Builder)
    );
    maybe_load_asset_atlas(
        &mut atlas_handles.enemy_id,
        &sprite_handles.enemy_handle,
        &asset_server,
        &mut texture_atlases,
        data::get_asset_info(AssetType::Enemy)
    );
    maybe_load_asset_atlas(
        &mut atlas_handles.conveyor_id,
        &sprite_handles.conveyor_handle,
        &asset_server,
        &mut texture_atlases,
        data::get_asset_info(AssetType::Conveyor)
    );

    if atlas_handles.loaded() {
        sprite_handles.loaded = true;
    }
}

fn load_biome_atlases(
    atlas_handles: &mut ResMut<AtlasHandles>,
    asset_server: &Res<AssetServer>,
    sprite_handles: &ResMut<SpriteHandles>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    if !atlas_handles.biomes_loaded() {
        if are_assets_loaded(&sprite_handles.biome_handles, asset_server) {
            atlas_handles
                .grassland_biome_id
                .replace(load_biome_atlas(Biome::Grassland, asset_server, texture_atlases));

            atlas_handles
                .desert_biome_id
                .replace(load_biome_atlas(Biome::Desert, asset_server, texture_atlases));

            atlas_handles
                .rocklands_biome_id
                .replace(load_biome_atlas(Biome::Rockland, asset_server, texture_atlases));
        }
    }
}

fn are_assets_loaded(
    sprite_handles: &Vec<HandleUntyped>,
    asset_server: &Res<AssetServer>,
) -> bool {
    asset_server.get_group_load_state(sprite_handles.iter().map(|handle| handle.id)) == LoadState::Loaded
}

fn load_asset_atlas_of_group(
    asset_info: AssetInfo,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> HandleId {
    let asset_handle = asset_server.get_handle(asset_info.sprite_file.as_str());
    let biome_atlas = TextureAtlas::from_grid(
        asset_handle, asset_info.tile_size, asset_info.columns, asset_info.rows
    );
    texture_atlases.add(biome_atlas).id
}

fn load_biome_atlas(
    biome: Biome,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> HandleId {
    load_asset_atlas_of_group(data::get_biome_asset_info(biome), asset_server, texture_atlases)
}

fn load_asset_atlas(
    sprite_handle: &Handle<Texture>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_info: AssetInfo,
) -> Option<HandleId> {
    let sprite_asset_handle = asset_server.get_handle(sprite_handle);
    if let LoadState::Loaded = asset_server.get_load_state(&sprite_asset_handle) {
        let atlas = TextureAtlas::from_grid(
            sprite_asset_handle, asset_info.tile_size, asset_info.columns, asset_info.rows
        );
        let atlas_handle = texture_atlases.add(atlas);
        Option::Some(atlas_handle.id)
    } else {
        Option::None
    }
}

fn maybe_load_asset_atlas(
    atlas_handle: &mut Option<HandleId>,
    sprite_handle: &Handle<Texture>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_info: AssetInfo,
) {
    if !atlas_handle.is_some() {
        if let Some(handle_id) = load_asset_atlas(sprite_handle, asset_server, texture_atlases, asset_info) {
            atlas_handle.replace(handle_id);
        }
    }
}