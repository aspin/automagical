use crate::biome::Biome;
use crate::global_constants::TILE_LENGTH;
use crate::inventory::ItemType;
use bevy::math::Vec2;

pub struct AssetInfo {
    pub sprite_file: String,
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
}

impl AssetInfo {
    pub fn new(sprite_file: String, tile_size: Vec2, columns: usize, rows: usize) -> Self {
        AssetInfo {
            sprite_file,
            tile_size,
            columns,
            rows,
        }
    }
}

pub struct AssetGroupInfo {
    pub folder_path: String,
    pub assets_info: Vec<(AssetType, AssetInfo)>,
}

impl AssetGroupInfo {
    pub fn new(folder_path: String, assets_info: Vec<(AssetType, AssetInfo)>) -> Self {
        AssetGroupInfo {
            folder_path,
            assets_info,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum AssetType {
    Arrow,
    Builder,
    Enemy,
    Conveyor,
    Grassland,
    Desert,
    Rockland,
    Font,
}

impl From<Biome> for AssetType {
    fn from(biome_type: Biome) -> Self {
        match biome_type {
            Biome::Grassland => Self::Grassland,
            Biome::Desert => Self::Desert,
            Biome::Rockland => Self::Rockland,
        }
    }
}

impl From<ItemType> for AssetType {
    fn from(item_type: ItemType) -> Self {
        match item_type {
            ItemType::Conveyor => Self::Conveyor,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum AssetGroup {
    Biome,
    Projectile,
}

pub fn default_tile_size() -> Vec2 {
    Vec2::new(TILE_LENGTH as f32, TILE_LENGTH as f32)
}

pub fn all_asset_groups() -> Vec<AssetGroup> {
    vec![AssetGroup::Biome, AssetGroup::Projectile]
}

pub fn all_base_assets() -> Vec<AssetType> {
    vec![AssetType::Builder, AssetType::Enemy, AssetType::Conveyor]
}

fn grassland_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/biome/grass.png"),
        default_tile_size(),
        4,
        1,
    )
}

fn desert_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/biome/desert.png"),
        default_tile_size(),
        4,
        1,
    )
}

fn rocklands_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/biome/rocklands.png"),
        default_tile_size(),
        4,
        1,
    )
}

fn arrow_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/projectile/arrow.png"),
        Vec2::new(8., 4.),
        1,
        1,
    )
}

fn builder_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/wizard.png"),
        default_tile_size(),
        7,
        3,
    )
}

fn enemy_asset_info() -> AssetInfo {
    AssetInfo::new(String::from("texture/enemy.png"), default_tile_size(), 7, 3)
}

fn conveyor_asset_info() -> AssetInfo {
    AssetInfo::new(
        String::from("texture/conveyor.png"),
        default_tile_size(),
        1,
        1,
    )
}

fn font_asset_info() -> AssetInfo {
    // TODO: most of this is not necessary
    AssetInfo::new(String::from("font/square.ttf"), default_tile_size(), 0, 0)
}

pub fn get_asset_info(asset_type: AssetType) -> AssetInfo {
    match asset_type {
        AssetType::Builder => builder_asset_info(),
        AssetType::Enemy => enemy_asset_info(),
        AssetType::Arrow => arrow_asset_info(),
        AssetType::Conveyor => conveyor_asset_info(),
        AssetType::Grassland => grassland_asset_info(),
        AssetType::Desert => desert_asset_info(),
        AssetType::Rockland => rocklands_asset_info(),
        AssetType::Font => font_asset_info(),
    }
}

pub fn get_asset_group_info(asset_group_type: AssetGroup) -> AssetGroupInfo {
    match asset_group_type {
        AssetGroup::Biome => AssetGroupInfo::new(
            String::from("texture/biome"),
            vec![
                (AssetType::Grassland, grassland_asset_info()),
                (AssetType::Desert, desert_asset_info()),
                (AssetType::Rockland, rocklands_asset_info()),
            ],
        ),
        AssetGroup::Projectile => AssetGroupInfo::new(
            String::from("texture/projectile"),
            vec![(AssetType::Arrow, arrow_asset_info())],
        ),
    }
}

pub fn get_asset_sprite_path(asset_type: AssetType) -> String {
    get_asset_info(asset_type).sprite_file
}
