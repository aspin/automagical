use crate::inventory::item_slot::{ItemSlot, draw_item_slot};
use crate::global_constants::INVENTORY_SIZE;
use bevy::prelude::*;

pub struct PlayerInventory {
    pub items: [ItemSlot; INVENTORY_SIZE],
    pub show: bool
}

impl PlayerInventory {
    pub fn new(items: [ItemSlot; 50], show: bool) -> Self {
        PlayerInventory { items, show }
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        let mut items = [ItemSlot::empty(); INVENTORY_SIZE];
        items[0] = ItemSlot::conveyors(10);
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
            for _ in 0..INVENTORY_SIZE {
                draw_item_slot(parent, &mut materials);
            }
        });
}

pub fn draw_inventory(
    player_inventory: &PlayerInventory,
    mut inventory_style: Mut<Style>
) {
    if player_inventory.show {
        inventory_style.display = Display::Flex
    } else {
        inventory_style.display = Display::None
    }
}