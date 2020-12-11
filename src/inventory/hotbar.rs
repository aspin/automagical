use crate::asset_loader::SpriteHandles;
use crate::global_constants::HOTBAR_LENGTH;
use crate::inventory::item_slot::{
    draw_item_slot, set_item_slot_icon, set_item_slot_text, ItemSlot,
};
use crate::inventory::{MaterialHandles, PlayerInventory};
use bevy::prelude::*;

#[derive(Debug)]
pub struct Hotbar {
    pub items: [HotbarItemSlot; HOTBAR_LENGTH],
}

impl Hotbar {
    pub fn new(items: [HotbarItemSlot; HOTBAR_LENGTH]) -> Self {
        Hotbar { items }
    }
}

impl Default for Hotbar {
    fn default() -> Self {
        let mut items = [HotbarItemSlot::empty(); HOTBAR_LENGTH];
        items[0] = HotbarItemSlot::new(Some(0));
        Hotbar::new(items)
    }
}

pub struct HotbarIndex {
    index: usize,
}

impl HotbarIndex {
    pub fn new(index: usize) -> Self {
        HotbarIndex { index }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HotbarItemSlot {
    inventory_index: Option<usize>,
}

impl HotbarItemSlot {
    pub fn new(inventory_index: Option<usize>) -> Self {
        HotbarItemSlot { inventory_index }
    }

    pub fn empty() -> Self {
        HotbarItemSlot::new(Option::None)
    }
}

pub fn setup_hotbar(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(0.),
                    left: Val::Percent(0.),
                    ..Default::default()
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Px(200.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgba(0.5, 0.5, 0.5, 0.8).into()),
                    ..Default::default()
                })
                .with(Hotbar::default())
                .with_children(|parent| {
                    for i in 0..HOTBAR_LENGTH {
                        draw_item_slot(parent, &mut materials).with(HotbarIndex::new(i));
                    }
                });
        });
}

// TODO: this probably doesn't need to be drawn on each iteration?
// Can probably rely on other events to trigger.
pub fn draw_hotbar(
    sprite_handles: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut material_handles: ResMut<MaterialHandles>,
    hotbar_query: Query<&Hotbar>,
    hotbar_index_query: Query<(&HotbarIndex, &Children)>,
    mut material_query: Query<&mut Handle<ColorMaterial>>,
    mut text_query: Query<&mut Text>,
    inventory_query: Query<&PlayerInventory>,
) {
    if !sprite_handles.loaded {
        return;
    }

    if let Some(player_inventory) = inventory_query.iter().next() {
        let hotbar = hotbar_query.iter().next().unwrap();
        for (hotbar_index, children) in hotbar_index_query.iter() {
            for child_entity in children.0.iter() {
                if let Ok(color_handle) = material_query.get_mut(*child_entity) {
                    if let Some(item_slot) =
                        get_inventory_item(player_inventory, hotbar, hotbar_index.index)
                    {
                        set_item_slot_icon(
                            color_handle,
                            item_slot,
                            &sprite_handles,
                            &asset_server,
                            &mut materials,
                            &mut material_handles,
                        );
                    }
                }
                if let Ok(text) = text_query.get_mut(*child_entity) {
                    if let Some(item_slot) =
                        get_inventory_item(player_inventory, hotbar, hotbar_index.index)
                    {
                        set_item_slot_text(text, item_slot, &sprite_handles, &asset_server);
                    }
                }
            }
        }
    }
}

fn get_inventory_item(
    player_inventory: &PlayerInventory,
    hotbar: &Hotbar,
    hotbar_index: usize,
) -> Option<ItemSlot> {
    if let Some(inventory_index) = hotbar.items[hotbar_index].inventory_index {
        Some(player_inventory.items[inventory_index])
    } else {
        Option::None
    }
}
