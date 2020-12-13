use crate::global_constants::INVENTORY_SIZE;
use bevy::prelude::*;
use crate::inventory::ItemSlot;

pub struct PlayerInventory {
    pub items: [ItemSlot; INVENTORY_SIZE],
    pub show: bool,
    ui_entity: Entity,
}

impl PlayerInventory {
    pub fn new(items: [ItemSlot; INVENTORY_SIZE], show: bool, ui_entity: Entity) -> Self {
        PlayerInventory {
            items,
            show,
            ui_entity,
        }
    }

    pub fn empty(ui_entity: Entity) -> Self {
        let mut items = [ItemSlot::empty(); INVENTORY_SIZE];
        items[0] = ItemSlot::conveyors(9);
        PlayerInventory::new(items, false, ui_entity)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn ui_entity(&self) -> Entity {
        self.ui_entity
    }
}

pub struct InventoryIndex {
    index: usize,
}

impl InventoryIndex {
    pub fn new(index: usize) -> Self {
        InventoryIndex { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
