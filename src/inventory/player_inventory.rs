use crate::inventory::item_slot::ItemSlot;
use crate::global_constants::INVENTORY_SIZE;

pub struct PlayerInventory {
    pub items: [ItemSlot; INVENTORY_SIZE],
    pub show: bool
}
