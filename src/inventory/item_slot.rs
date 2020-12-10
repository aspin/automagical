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
    commands: &'a mut ChildBuilder<'data>, materials: &mut ResMut<Assets<ColorMaterial>>
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
        })
}