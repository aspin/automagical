use bevy::ecs::{ResMut, Res};
use bevy::asset::{Assets, AssetServer, HandleId};
use bevy::sprite::TextureAtlas;
use crate::data;

#[derive(Debug, PartialEq)]
pub enum Biome {
    Grassland,
    Desert,
    Rockland
}

pub fn load_biome_atlas(
    biome: Biome,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> HandleId {
    let asset_data = data::get_biome_asset_info(biome);

    let asset_handle = asset_server.get_handle(asset_data.sprite_file.as_str());
    let biome_atlas = TextureAtlas::from_grid(
        asset_handle, asset_data.tile_size, asset_data.columns, asset_data.rows
    );
    texture_atlases.add(biome_atlas).id
}
