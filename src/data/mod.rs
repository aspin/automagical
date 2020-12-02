/*
The majority of code in this module should eventually be turned into data-files / prefabs.
 */

mod animation_data;
mod asset_data;

pub use animation_data::get_animation_info;
pub use asset_data::get_asset_info;
pub use asset_data::get_asset_group_info;
pub use asset_data::get_asset_sprite_path;
pub use asset_data::default_tile_size;
pub use asset_data::AssetType;
pub use asset_data::AssetInfo;
pub use asset_data::AssetGroup;
pub use asset_data::AssetGroupInfo;
