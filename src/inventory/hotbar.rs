use crate::asset_loader::SpriteHandles;
use crate::data::AssetType;
use crate::global_constants::HOTBAR_LENGTH;
use crate::inventory::item_slot::{ItemSlot, draw_item_slot};
use crate::inventory::MaterialHandles;
use bevy::prelude::*;

#[derive(Debug)]
pub struct Hotbar {
    pub items: [ItemSlot; HOTBAR_LENGTH],
}

impl Hotbar {
    pub fn new(items: [ItemSlot; HOTBAR_LENGTH]) -> Self {
        Hotbar { items }
    }
}

impl Default for Hotbar {
    fn default() -> Self {
        let mut items = [ItemSlot::empty(); HOTBAR_LENGTH];
        items[0] = ItemSlot::conveyors(10);
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
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
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
                        draw_item_slot(parent, &mut materials)
                            .with(HotbarIndex::new(i));
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
) {
    if !sprite_handles.loaded {
        return;
    }

    let hotbar = hotbar_query.iter().next().unwrap();
    for (hotbar_index, children) in hotbar_index_query.iter() {
        for child_entity in children.0.iter() {
            if let Ok(color_handle) = material_query.get_mut(*child_entity) {
                set_hotbar_item_icon(
                    color_handle,
                    hotbar.items[hotbar_index.index],
                    &sprite_handles,
                    &asset_server,
                    &mut materials,
                    &mut material_handles
                );
            }
            if let Ok(text) = text_query.get_mut(*child_entity) {
                set_hotbar_text(
                    text,
                    hotbar.items[hotbar_index.index],
                    &sprite_handles,
                    &asset_server
                );
            }
        }
    }
}

fn set_hotbar_item_icon(
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

fn set_hotbar_text(
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