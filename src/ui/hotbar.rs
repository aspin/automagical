use crate::asset_loader::SpriteHandles;
use crate::inventory::{Hotbar, HotbarIndex, ItemSlot, PlayerInventory};
use crate::ui::item_slot::{draw_item_slot, set_item_slot_icon, set_item_slot_text};
use crate::ui::MaterialHandles;
use bevy::prelude::*;

/// Draw hotbar UI element.
pub(super) fn setup_hotbar(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hotbar: Res<Hotbar>,
) {
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
                .with_children(|parent| {
                    for i in 0..hotbar.len() {
                        draw_item_slot(parent, &mut materials).with(HotbarIndex::new(i));
                    }
                });
        });
}

/// Update hotbar UI element. Draws the stored item icon and stack count.
/// TODO: this probably doesn't need to be drawn on each iteration? (other events to trigger?)
pub(super) fn draw_hotbar(
    sprite_handles: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut material_handles: ResMut<MaterialHandles>,
    hotbar: Res<Hotbar>,
    inventory: Res<PlayerInventory>,
    hotbar_index_query: Query<(&HotbarIndex, &Children)>,
    mut material_query: Query<&mut Handle<ColorMaterial>>,
    mut text_query: Query<&mut Text>,
) {
    if !sprite_handles.loaded {
        return;
    }

    for (hotbar_index, children) in hotbar_index_query.iter() {
        for child_entity in children.0.iter() {
            if let Ok(color_handle) = material_query.get_mut(*child_entity) {
                if let Some(item_slot) =
                    get_inventory_item(&inventory, &hotbar, hotbar_index.index())
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
                    get_inventory_item(&inventory, &hotbar, hotbar_index.index())
                {
                    set_item_slot_text(text, item_slot, &sprite_handles, &asset_server);
                }
            }
        }
    }
}

fn get_inventory_item(
    inventory: &Res<PlayerInventory>,
    hotbar: &Res<Hotbar>,
    hotbar_index: usize,
) -> Option<ItemSlot> {
    if let Some(inventory_index) = hotbar.items[hotbar_index].index() {
        Some(inventory.items[inventory_index])
    } else {
        Option::None
    }
}
