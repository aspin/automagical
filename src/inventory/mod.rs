mod hotbar;
mod item_slot;
mod player_inventory;

use bevy::prelude::*;

use std::collections::HashMap;
use crate::data::AssetType;

pub use item_slot::ItemType;
pub use player_inventory::PlayerInventory;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MaterialHandles>()
            .add_startup_system(setup_camera.system())
            .add_startup_system(hotbar::setup_hotbar.system())
            .add_startup_system(player_inventory::setup_inventory.system())
            .add_system(hotbar::draw_hotbar.system())
            .add_system(player_inventory::draw_inventory.system());
    }
}

#[derive(Default)]
pub struct MaterialHandles {
    material_handles: HashMap<AssetType, Handle<ColorMaterial>>,
}

impl MaterialHandles {
    pub fn get(&self, asset_type: AssetType) -> Option<&Handle<ColorMaterial>> {
        self.material_handles.get(&asset_type)
    }

    pub fn insert(
        &mut self,
        asset_type: AssetType,
        material_handle: Handle<ColorMaterial>,
    ) -> Option<Handle<ColorMaterial>> {
        self.material_handles.insert(asset_type, material_handle)
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(UiCameraComponents::default());
}
