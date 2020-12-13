use crate::asset_loader::SpriteHandles;
use crate::data::AssetType;
use crate::inventory::ItemSlot;
use crate::ui::MaterialHandles;
use bevy::prelude::*;

/// Draws an item slot UI element.
/// Item slot contains both the image icon and item count.
pub(super) fn draw_item_slot<'a, 'data>(
    commands: &'a mut ChildBuilder<'data>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> &'a mut ChildBuilder<'data> {
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                margin: Rect::all(Val::Px(10.0)),
                ..Default::default()
            },
            material: materials.add(Color::RED.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .spawn(TextComponents {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(0.),
                            right: Val::Px(0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "".to_string(),
                        style: TextStyle {
                            color: Color::WHITE,
                            font_size: 30.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                });
        })
}

/// Sets the item icon for a slot
/// # Arguments
/// * `color_handle` - Material handle of item slot icon ImageComponents
/// * `item_slot` - Item slot information from inventory
/// * `sprite_handles` - Loaded sprite handle resources
/// * `asset_server` - Asset server resource TODO: see if can remove?
/// * `materials` - Resource of all material assets
/// * `material_handles` - Resource of all loaded material asset handles
pub(super) fn set_item_slot_icon(
    mut color_handle: Mut<Handle<ColorMaterial>>,
    item_slot: ItemSlot,
    sprite_handles: &Res<SpriteHandles>,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    material_handles: &mut ResMut<MaterialHandles>,
) {
    if let Some(item_type) = item_slot.item_type {
        let asset_type = AssetType::from(item_type);
        if let Some(material_handle) = material_handles.get(asset_type) {
            color_handle.id = material_handle.id
        } else {
            let item_handle = sprite_handles.get_asset(asset_type).unwrap();
            let sprite_asset_handle = asset_server.get_handle(item_handle);
            let material_handle = materials.add(sprite_asset_handle.into());
            color_handle.id = material_handle.id;
            material_handles.insert(asset_type, material_handle);
        }
    }
}

/// Sets the item icon for a slot
/// # Arguments
/// * `text` - Text handle of item slot text TextComponents
/// * `item_slot` - Item slot information from inventory
/// * `sprite_handles` - Loaded sprite handle resources
/// * `asset_server` - Asset server resource TODO: see if can remove?
pub(super) fn set_item_slot_text(
    mut text: Mut<Text>,
    item_slot: ItemSlot,
    sprite_handles: &Res<SpriteHandles>,
    asset_server: &Res<AssetServer>,
) {
    if let Some(item_count) = item_slot.count {
        let font_handle = sprite_handles.get_asset(AssetType::Font).unwrap();

        let font_asset_handle = asset_server.get_handle(font_handle);
        text.font = font_asset_handle;
        text.value = item_count.to_string();
    }
}
