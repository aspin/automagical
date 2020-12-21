mod hotbar;
mod inventory;
mod item_slot;
mod material_handles;

use bevy::prelude::*;

pub use material_handles::MaterialHandles;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MaterialHandles>()
            .add_startup_system(setup_camera.system())
            .add_startup_system(hotbar::setup_hotbar.system())
            .add_startup_system(inventory::setup_inventory.system())
            .add_system(hotbar::draw_hotbar.system())
            .add_system(inventory::draw_inventory.system());
    }
}

fn setup_camera(commands: &mut Commands) {
    commands.spawn(CameraUiBundle::default());
}
