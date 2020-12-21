use crate::asset_loader::AtlasHandles;
use crate::builder::{BuilderMode, Player};
use crate::cursor::CursorState;
use crate::data::AssetType;
use crate::world_map::WorldMap;
use bevy::prelude::*;

pub fn place_object(
    commands: &mut Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_state: Res<CursorState>,
    world_map: Res<WorldMap>,
    atlas_handles: Res<AtlasHandles>,
    player: Res<Player>,
) {
    if player.mode != BuilderMode::Construct {
        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        if let Some(cursor_coordinates) = cursor_state.cursor_position {
            if let Some(conveyor_id) = atlas_handles.get_asset(AssetType::Conveyor) {
                let map_tile =
                    world_map.position_to_tile(cursor_coordinates.x, cursor_coordinates.y);
                let tile_position = world_map.tile_to_position(map_tile.0, map_tile.1);
                let conveyor_atlas_handle = Handle::weak(conveyor_id);

                println!("Spawning conveyor at {:?} {:?}", map_tile, tile_position);

                commands.spawn(SpriteSheetBundle {
                    texture_atlas: conveyor_atlas_handle,
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_translation(Vec3::new(
                        tile_position.translation.x,
                        tile_position.translation.y,
                        2.,
                    )),
                    ..Default::default()
                });
            }
        }
    }
}
