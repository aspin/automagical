mod hotbar;
mod item_slot;
mod player_inventory;

use bevy::prelude::*;

pub use hotbar::Hotbar;
pub use hotbar::HotbarIndex;
pub use item_slot::ItemSlot;
pub use item_slot::ItemType;
pub use player_inventory::InventoryIndex;
pub use player_inventory::PlayerInventory;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Hotbar>();
    }
}
