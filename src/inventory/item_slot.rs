use crate::asset_loader::SpriteHandles;
use crate::data::AssetType;
use crate::inventory::MaterialHandles;
use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct ItemSlot {
    pub item_type: Option<ItemType>,
    pub count: Option<u32>,
}

impl ItemSlot {
    pub fn new(item_type: ItemType, count: u32) -> Self {
        ItemSlot {
            item_type: Some(item_type),
            count: Some(count),
        }
    }

    pub fn conveyors(count: u32) -> Self {
        Self::new(ItemType::Conveyor, count)
    }

    pub fn empty() -> Self {
        ItemSlot {
            item_type: Option::None,
            count: Option::None,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum ItemType {
    Conveyor,
}

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
