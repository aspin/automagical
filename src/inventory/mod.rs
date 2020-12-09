mod hotbar;
mod item_slot;

use bevy::prelude::*;

pub use item_slot::ItemType;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_camera.system())
            .add_startup_system(hotbar::setup_hotbar.system())
            .add_system(hotbar::draw_hotbar.system());
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(UiCameraComponents::default());
}
