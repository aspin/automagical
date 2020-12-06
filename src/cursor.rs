use bevy::app::{EventReader, Events};
use bevy::window::{CursorMoved, Windows};
use bevy::prelude::{Entity, Transform, Vec2};
use bevy::math::f32::Vec4;
use bevy::ecs::{ResMut, Res, Query};

pub struct CursorState {
    pub cursor: EventReader<CursorMoved>,
    pub camera_entity: Entity,
    pub cursor_position: Option<Vec4>,
    pub world_position: Option<Vec4>,
}

pub fn update_cursor_position(
    mut cursor_state: ResMut<CursorState>,
    events_cursor: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    camera_query: Query<&Transform>,
) {
    let camera_transform = camera_query
        .get_component::<Transform>(cursor_state.camera_entity)
        .unwrap();

    // from cookbook: https://github.com/jamadazi/bevy-cookbook/blob/master/bevy-cookbook.md#2d-games
    for event in cursor_state.cursor.iter(&events_cursor) {
        let window = windows.get(event.id).unwrap();
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        let position = (event.position - size / 2.0).extend(0.0).extend(1.0);
        cursor_state.cursor_position.replace(position);
    }

    if let Some(cursor_position) = cursor_state.cursor_position {
        cursor_state.world_position.replace(camera_transform.compute_matrix() * cursor_position);
    }
}
