use crate::asset_loader::SpriteHandles;
use crate::data::AssetType;
use crate::global_constants::HOTBAR_LENGTH;
use crate::inventory::item_slot::ItemSlot;
use bevy::prelude::*;
use crate::inventory::MaterialHandles;

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

pub fn setup_hotbar(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                    let mut parent_reference = parent;
                    for i in 0..HOTBAR_LENGTH {
                        parent_reference = parent_reference
                            .spawn(NodeComponents {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    margin: Rect::all(Val::Px(10.0)),
                                    ..Default::default()
                                },
                                material: materials.add(Color::RED.into()),
                                ..Default::default()
                            })
                            .with(HotbarIndex::new(i))
                            .with_children(|parent| {
                                parent
                                    .spawn(ImageComponents {
                                        style: Style {
                                            size: Size::new(
                                                Val::Percent(100.0),
                                                Val::Percent(100.0),
                                            ),
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
                            });
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
            if let Ok(mut color_handle) = material_query.get_mut(*child_entity) {
                let item_slot = hotbar.items[hotbar_index.index];
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
            if let Ok(mut text) = text_query.get_mut(*child_entity) {
                let item_slot = hotbar.items[hotbar_index.index];
                if let Some(item_count) = item_slot.count {
                    let font_handle = sprite_handles
                        .get_asset(AssetType::Font)
                        .unwrap();

                    let font_asset_handle = asset_server.get_handle(font_handle);
                    text.font = font_asset_handle;
                    text.value = item_count.to_string();
                }
            }
        }
    }
}
