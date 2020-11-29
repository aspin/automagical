use bevy::prelude::*;
use crate::world_renderer::{WORLD_MAP_RENDER_WIDTH, WORLD_MAP_RENDER_HEIGHT};
use crate::asset_loader::{AtlasHandles, TILE_LENGTH};

pub const WORLD_MAP_WIDTH: usize = 300;
pub const WORLD_MAP_HEIGHT: usize = 300;

pub struct TileCoordinate(pub usize, pub usize);

pub struct WorldMap {
    tiles: Vec<Tile>,

    // in number of tiles
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub enum Biome {
    Grassland,
    Desert
}

#[derive(Debug)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub biome: Biome,
    pub rendered_entity: Option<Entity>,
}

impl WorldMap {
    fn new(width: usize, height: usize) -> WorldMap {
        let mut tiles: Vec<Tile> = Vec::with_capacity(width * height);
        for y in 0..width {
            for x in 0..height {
                tiles.push(Tile::new(x, y));
            }
        }
        WorldMap { tiles, width, height }
    }

    #[allow(dead_code)]
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y * self.width + x)
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y * self.width + x)
    }

    pub fn center_tile(&self) -> TileCoordinate {
        TileCoordinate(self.width / 2 as usize, self.height / 2 as usize)
    }

    pub fn position_to_tile(&self, x: f32, y: f32) -> TileCoordinate {
        let center = self.center_tile();
        let x_offset = (x / TILE_LENGTH as f32) as i32;
        let y_offset = (y / TILE_LENGTH as f32) as i32;
        TileCoordinate((center.0 as i32 + x_offset) as usize, (center.1 as i32 + y_offset) as usize)
    }

    pub fn tile_to_position(&self, x: usize, y: usize) -> Transform {
        tile_to_position(&self.center_tile(), x, y)
    }

    pub fn get_tiles_for_update(&mut self, camera_x: f32, camera_y: f32) -> (Vec<&mut Tile>, Vec<&mut Tile>) {
        let mut tiles_to_render: Vec<&mut Tile> = Vec::new();
        let mut tiles_to_despawn: Vec<&mut Tile> = Vec::new();

        let central_tile = self.position_to_tile(camera_x, camera_y);

        // println!("Builder coordinate: {} {}", central_tile.0, central_tile.1);

        let left_x = central_tile.0 - WORLD_MAP_RENDER_WIDTH;
        let right_x = central_tile.0 + WORLD_MAP_RENDER_WIDTH;
        let top_y = central_tile.1 + WORLD_MAP_RENDER_HEIGHT;
        let bot_y = central_tile.1 - WORLD_MAP_RENDER_HEIGHT;

        // println!("render in box: x{}-{}, y{}-{}", left_x, right_x, bot_y, top_y);

        for tile in self.tiles.iter_mut() {
            let render_tile = tile.x >= left_x && tile.x <= right_x && tile.y <= top_y && tile.y >= bot_y;
            if tile.rendered_entity.is_some() {
                if !render_tile {
                    tiles_to_despawn.push(tile);
                }
            } else {
                if render_tile {
                    tiles_to_render.push(tile);
                }
            }
        }

        (tiles_to_render, tiles_to_despawn)
    }
}

impl FromResources for WorldMap {
    fn from_resources(_resources: &Resources) -> Self {
        WorldMap::new(WORLD_MAP_WIDTH, WORLD_MAP_HEIGHT)
    }
}

impl Tile {
    fn new(x: usize, y: usize) -> Tile {
        Tile { x, y, biome: Biome::Grassland, rendered_entity: Option::None }
    }

    pub fn get_biome_handle(&self, atlas_handles: &AtlasHandles) -> Handle<TextureAtlas> {
        let handle_id = match self.biome {
            Biome::Grassland => atlas_handles.grassland_biome_id.unwrap(),
            Biome::Desert => atlas_handles.desert_biome_id.unwrap()
        };
        Handle::weak(handle_id)
    }
}

pub fn tile_to_position(center_tile: &TileCoordinate, x: usize, y: usize) -> Transform {
    Transform::from_translation(
        Vec3::new(
            ((x as i32 - center_tile.0 as i32) * TILE_LENGTH as i32) as f32,
            ((y as i32 - center_tile.1 as i32) * TILE_LENGTH as i32) as f32,
            0.
        )
    )
}

