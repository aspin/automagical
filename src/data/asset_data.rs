use bevy::math::Vec2;
use crate::global_constants::TILE_LENGTH;
use crate::biome::Biome;

pub struct AssetInfo {
    pub sprite_file: String,
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
}

pub enum AssetType {
    Arrow,
    Builder,
    Enemy,
    Conveyor
}

impl AssetInfo {
    pub fn new(sprite_file: String, tile_size: Vec2, columns: usize, rows: usize) -> Self {
        AssetInfo { sprite_file, tile_size, columns, rows }
    }
}

pub fn default_tile_size() -> Vec2 {
    Vec2::new(TILE_LENGTH as f32, TILE_LENGTH as f32)
}

fn grassland_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/biome/grass.png"), default_tile_size(), 4, 1
    )
}

fn desert_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/biome/desert.png"), default_tile_size(), 4, 1
    )
}

fn rocklands_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/biome/rocklands.png"), default_tile_size(), 4, 1
    )
}

fn arrow_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/projectile/arrow.png"), Vec2::new(8., 4.), 1, 1
    )
}

fn builder_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/wizard.png"), default_tile_size(), 7, 3
    )
}

fn enemy_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/enemy.png"), default_tile_size(), 7, 3
    )
}

fn conveyor_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/conveyor.png"), default_tile_size(), 1, 1
    )
}

pub fn get_biome_asset_info(biome: Biome) -> AssetInfo {
    match biome {
        Biome::Grassland => grassland_asset_info(),
        Biome::Desert => desert_asset_info(),
        Biome::Rockland => rocklands_asset_info(),
    }
}

pub fn get_asset_info(asset_type: AssetType) -> AssetInfo {
    match asset_type {
        AssetType::Builder => builder_asset_info(),
        AssetType::Enemy => enemy_asset_info(),
        AssetType::Arrow => arrow_asset_info(),
        AssetType::Conveyor => conveyor_asset_info(),
    }
}

pub fn get_asset_sprite_path(asset_type: AssetType) -> String {
    get_asset_info(asset_type).sprite_file
}