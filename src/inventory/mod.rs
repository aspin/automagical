mod hotbar;
mod item_slot;

use bevy::prelude::*;

pub use item_slot::ItemType;
use std::collections::HashMap;
use crate::data::AssetType;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<MaterialHandles>()
            .add_startup_system(setup_camera.system())
            .add_startup_system(hotbar::setup_hotbar.system())
            .add_system(hotbar::draw_hotbar.system());
    }
}

#[derive(Default)]
pub struct MaterialHandles {
    material_handles: HashMap<AssetType, Handle<ColorMaterial>>
}

impl MaterialHandles {
    pub fn get(&self, asset_type: AssetType) -> Option<&Handle<ColorMaterial>> {
        self.material_handles.get(&asset_type)
    }

    pub fn insert(
        &mut self, asset_type: AssetType, material_handle: Handle<ColorMaterial>
    ) -> Option<Handle<ColorMaterial>> {
        self.material_handles.insert(asset_type, material_handle)
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(UiCameraComponents::default());
}
