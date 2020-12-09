use std::collections::HashMap;

use bevy::asset::{HandleId, LoadState};
use bevy::prelude::*;

use crate::biome::Biome;
use crate::cursor::CursorState;
use crate::data;
use crate::data::{AssetGroup, AssetInfo, AssetType};
use crate::global_constants::CAMERA_ZOOM;

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
    asset_handles: HashMap<AssetType, Handle<Texture>>,
    asset_group_handles: HashMap<AssetGroup, Vec<HandleUntyped>>,
    pub loaded: bool,
}

impl SpriteHandles {
    fn add_asset(&mut self, asset_type: AssetType, asset_server: &AssetServer) {
        let sprite_path = data::get_asset_sprite_path(asset_type);
        self.asset_handles
            .insert(asset_type, asset_server.load(sprite_path.as_str()));
    }

    fn add_asset_group(&mut self, asset_group: AssetGroup, asset_server: &AssetServer) {
        let asset_group_info = data::get_asset_group_info(asset_group);
        let asset_handles = asset_server
            .load_folder(asset_group_info.folder_path.as_str())
            .unwrap();
        self.asset_group_handles.insert(asset_group, asset_handles);
    }

    pub fn get_asset(&self, asset_type: AssetType) -> Option<&Handle<Texture>> {
        self.asset_handles.get(&asset_type)
    }

    pub fn get_asset_group(&self, asset_group: AssetGroup) -> Option<&Vec<HandleUntyped>> {
        self.asset_group_handles.get(&asset_group)
    }
}

#[derive(Default)]
pub struct AtlasHandles {
    handles: HashMap<AssetType, HandleId>,
    handle_groups: HashMap<AssetGroup, Vec<AssetType>>,
}

impl AtlasHandles {
    fn try_load_handle_group(
        &mut self,
        asset_group: AssetGroup,
        sprite_handles: &ResMut<SpriteHandles>,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        if !self.handle_groups.contains_key(&asset_group) {
            if let Some(asset_group_sprites) = sprite_handles.get_asset_group(asset_group) {
                if are_assets_loaded(asset_group_sprites, &asset_server) {
                    let asset_group_info = data::get_asset_group_info(asset_group);
                    let mut asset_group_handles =
                        Vec::with_capacity(asset_group_info.assets_info.len());
                    for (asset_type, _asset_info) in asset_group_info.assets_info {
                        self.load_handle_of_group(asset_type, asset_server, texture_atlases);
                        asset_group_handles.push(asset_type);
                    }
                    self.handle_groups.insert(asset_group, asset_group_handles);
                }
            }
        }
    }

    fn load_handle_of_group(
        &mut self,
        asset_type: AssetType,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> &HandleId {
        let asset_info = data::get_asset_info(asset_type);
        self.handles.insert(
            asset_type.clone(),
            load_asset_atlas_of_group(asset_info, asset_server, texture_atlases),
        );
        self.handles.get(&asset_type).unwrap()
    }

    fn try_load_handle(
        &mut self,
        asset_type: AssetType,
        sprite_handles: &ResMut<SpriteHandles>,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        if !self.handles.contains_key(&asset_type) {
            let sprite_handle = sprite_handles.get_asset(asset_type).unwrap();
            let asset_info = data::get_asset_info(asset_type);

            if let Some(handle_id) =
                load_asset_atlas(sprite_handle, asset_server, texture_atlases, asset_info)
            {
                self.handles.insert(asset_type, handle_id);
            }
        }
    }

    pub fn get_asset(&self, asset_type: AssetType) -> Option<HandleId> {
        if let Some(handle_id) = self.handles.get(&asset_type) {
            Option::Some(handle_id.clone())
        } else {
            Option::None
        }
    }

    pub fn get_biome_asset(&self, biome_type: Biome) -> Option<HandleId> {
        self.get_asset(AssetType::from(biome_type))
    }

    fn asset_loaded(&self, asset_type: AssetType) -> bool {
        self.handles.contains_key(&asset_type)
    }

    fn asset_group_loaded(&self, asset_group: AssetGroup) -> bool {
        self.handle_groups.contains_key(&asset_group)
    }

    pub fn loaded(&self) -> bool {
        self.biomes_loaded() && self.projectiles_loaded() && self.assets_loaded()
    }

    fn biomes_loaded(&self) -> bool {
        self.asset_group_loaded(AssetGroup::Biome)
    }

    fn projectiles_loaded(&self) -> bool {
        self.asset_group_loaded(AssetGroup::Projectile)
    }

    fn assets_loaded(&self) -> bool {
        self.asset_loaded(AssetType::Builder)
            && self.asset_loaded(AssetType::Conveyor)
            && self.asset_loaded(AssetType::Builder)
    }
}

fn loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_sprite_handles: ResMut<SpriteHandles>,
) {
    map_sprite_handles.add_asset_group(AssetGroup::Biome, &asset_server);
    map_sprite_handles.add_asset_group(AssetGroup::Projectile, &asset_server);

    map_sprite_handles.add_asset(AssetType::Builder, &asset_server);
    map_sprite_handles.add_asset(AssetType::Enemy, &asset_server);
    map_sprite_handles.add_asset(AssetType::Conveyor, &asset_server);

    let camera_entity = commands
        .spawn(Camera2dComponents {
            transform: Transform {
                scale: Vec3::new(CAMERA_ZOOM, CAMERA_ZOOM, CAMERA_ZOOM),
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
        world_position: Option::None,
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

    for asset_group in data::all_asset_groups() {
        atlas_handles.try_load_handle_group(
            asset_group,
            &sprite_handles,
            &asset_server,
            &mut texture_atlases,
        );
    }

    for asset_type in data::all_base_assets() {
        atlas_handles.try_load_handle(
            asset_type,
            &sprite_handles,
            &asset_server,
            &mut texture_atlases,
        );
    }

    if atlas_handles.loaded() {
        sprite_handles.loaded = true;
    }
}

fn are_assets_loaded(sprite_handles: &Vec<HandleUntyped>, asset_server: &Res<AssetServer>) -> bool {
    asset_server.get_group_load_state(sprite_handles.iter().map(|handle| handle.id))
        == LoadState::Loaded
}

fn load_asset_atlas_of_group(
    asset_info: AssetInfo,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> HandleId {
    let asset_handle = asset_server.get_handle(asset_info.sprite_file.as_str());
    let biome_atlas = TextureAtlas::from_grid(
        asset_handle,
        asset_info.tile_size,
        asset_info.columns,
        asset_info.rows,
    );
    texture_atlases.add(biome_atlas).id
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
            sprite_asset_handle,
            asset_info.tile_size,
            asset_info.columns,
            asset_info.rows,
        );
        let atlas_handle = texture_atlases.add(atlas);
        Option::Some(atlas_handle.id)
    } else {
        Option::None
    }
}
