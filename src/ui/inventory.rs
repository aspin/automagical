use crate::asset_loader::SpriteHandles;
use crate::inventory::{InventoryIndex, PlayerInventory};
use crate::ui::item_slot::{draw_item_slot, set_item_slot_icon, set_item_slot_text};
use crate::ui::MaterialHandles;
use bevy::prelude::*;

/// Draw inventory UI element and initialize inventory resource.
/// By default, inventory is not shown.
pub(super) fn setup_inventory(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = commands
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
        .current_entity()
        .unwrap();
    let player_inventory = PlayerInventory::empty(entity);
    commands.with_children(|parent| {
        for i in 0..player_inventory.len() {
            draw_item_slot(parent, &mut materials).with(InventoryIndex::new(i));
        }
    });
    commands.insert_resource(player_inventory);
}

/// Update inventory UI element. Checks if player inventory should be shown, and draws stored
/// item icons and stack counts.
pub(super) fn draw_inventory(
    sprite_handles: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut material_handles: ResMut<MaterialHandles>,
    player_inventory: Res<PlayerInventory>,
    mut styles_query: Query<&mut Style>,
    inventory_index_query: Query<(&InventoryIndex, &Children)>,
    mut material_query: Query<&mut Handle<ColorMaterial>>,
    mut text_query: Query<&mut Text>,
) {
    if !sprite_handles.loaded {
        return;
    }

    let mut player_inventory_style = styles_query.get_mut(player_inventory.ui_entity()).unwrap();
    if player_inventory.show {
        player_inventory_style.display = Display::Flex
    } else {
        player_inventory_style.display = Display::None
    }
    for (inventory_index, children) in inventory_index_query.iter() {
        for child_entity in children.0.iter() {
            if let Ok(color_handle) = material_query.get_mut(*child_entity) {
                set_item_slot_icon(
                    color_handle,
                    player_inventory.items[inventory_index.index()],
                    &sprite_handles,
                    &asset_server,
                    &mut materials,
                    &mut material_handles,
                );
            }
            if let Ok(text) = text_query.get_mut(*child_entity) {
                set_item_slot_text(
                    text,
                    player_inventory.items[inventory_index.index()],
                    &sprite_handles,
                    &asset_server,
                );
            }
        }
    }
}
