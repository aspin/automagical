use crate::asset_loader::SpriteHandles;
use crate::global_constants::INVENTORY_SIZE;
use crate::inventory::item_slot::{
    draw_item_slot, set_item_slot_icon, set_item_slot_text, ItemSlot,
};
use crate::inventory::MaterialHandles;
use bevy::prelude::*;

pub struct PlayerInventory {
    pub items: [ItemSlot; INVENTORY_SIZE],
    pub show: bool,
}

impl PlayerInventory {
    pub fn new(items: [ItemSlot; 50], show: bool) -> Self {
        PlayerInventory { items, show }
    }
}

pub struct InventoryIndex {
    index: usize,
}

impl InventoryIndex {
    pub fn new(index: usize) -> Self {
        InventoryIndex { index }
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        let mut items = [ItemSlot::empty(); INVENTORY_SIZE];
        items[0] = ItemSlot::conveyors(9);
        PlayerInventory::new(items, false)
    }
}

pub fn setup_inventory(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(1220.0), Val::Px(620.0)),
                margin: Rect::all(Val::Auto),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_wrap: FlexWrap::Wrap,
                display: Display::None,
                ..Default::default()
            },
            material: materials.add(Color::BLUE.into()),
            ..Default::default()
        })
        .with(PlayerInventory::default())
        .with_children(|parent| {
            for i in 0..INVENTORY_SIZE {
                draw_item_slot(parent, &mut materials).with(InventoryIndex::new(i));
            }
        });
}

// TODO: consider a refactor
// set PlayerInventory to be a resource, with a reference to the root inventory element
pub fn draw_inventory(
    sprite_handles: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut material_handles: ResMut<MaterialHandles>,
    mut material_query: Query<&mut Handle<ColorMaterial>>,
    mut player_inventory_query: Query<(&PlayerInventory, &mut Style)>,
    inventory_index_query: Query<(&InventoryIndex, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    if let Some((player_inventory, mut style)) = player_inventory_query.iter_mut().next() {
        if player_inventory.show {
            style.display = Display::Flex
        } else {
            style.display = Display::None
        }
        if !sprite_handles.loaded {
            return;
        }

        for (inventory_index, children) in inventory_index_query.iter() {
            for child_entity in children.0.iter() {
                if let Ok(color_handle) = material_query.get_mut(*child_entity) {
                    set_item_slot_icon(
                        color_handle,
                        player_inventory.items[inventory_index.index],
                        &sprite_handles,
                        &asset_server,
                        &mut materials,
                        &mut material_handles,
                    );
                }
                if let Ok(text) = text_query.get_mut(*child_entity) {
                    set_item_slot_text(
                        text,
                        player_inventory.items[inventory_index.index],
                        &sprite_handles,
                        &asset_server,
                    );
                }
            }
        }
    }
}
