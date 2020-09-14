use bevy::prelude::*;
use crate::world_map::WorldMap;
use crate::asset_loader::AtlasHandles;

pub struct CursorState {
    pub cursor: EventReader<CursorMoved>,
    pub camera_entity: Entity,
    pub cursor_position: Option<Vec4>
}

pub fn update_cursor_position(
    mut cursor_state: ResMut<CursorState>,
    events_cursor: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    camera_query: Query<&Transform>
) {
    let camera_transform = camera_query.get::<Transform>(
        cursor_state.camera_entity
    ).unwrap();

    // from cookbook: https://github.com/jamadazi/bevy-cookbook/blob/master/bevy-cookbook.md#2d-games
    for event in cursor_state.cursor.iter(
        &events_cursor
    ) {
        let window = windows.get(event.id).unwrap();
        let size = Vec2::new(window.width as f32, window.height as f32);

        let position = event.position - size / 2.0;
        let position_world = camera_transform.value * position.extend(0.0).extend(1.0);

        cursor_state.cursor_position.replace(position_world);
    }
}

pub fn place_object(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_state: Res<CursorState>,
    world_map: Res<WorldMap>,
    atlas_handles: Res<AtlasHandles>
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        if let Some(cursor_coordinates) = cursor_state.cursor_position {
            if let Some(conveyor_id) = atlas_handles.conveyor_id {
                let map_tile = world_map.position_to_tile(
                    cursor_coordinates.x(), cursor_coordinates.y()
                );
                let tile_position = world_map.tile_to_position(map_tile.0, map_tile.1);
                let conveyor_atlas_handle = Handle::from_id(conveyor_id);

                commands.spawn(
                    SpriteSheetComponents {
                        texture_atlas: conveyor_atlas_handle,
                        sprite: TextureAtlasSprite::new(0),
                        translation: Translation::new(tile_position.x(), tile_position.y(), 2.),
                        ..Default::default()
                    }
                );
            }
        }
    }
}