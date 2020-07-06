use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::renderer::{SpriteSheet, SpriteRender};
use crate::utils::constants::{TILE_SIDE_LENGTH, TILE_OFFSET};
use crate::components::physics::Coordinate;

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub placed_object: Option<Entity>,
}

impl Tile {
    pub fn new(x: usize, y: usize) -> Tile {
        Tile {
            x, y, placed_object: Option::None
        }
    }

    /// Generates a tile map to be used in the world map.
    /// For efficiency, vector is compacted in one dimensions.
    ///
    /// i = 0 => x = 0, y = 0
    /// i = 1 => x = 1, y = 0
    /// i = 2 => x = 2, y = 0
    /// etc.
    ///
    /// TODO: move this to world_map.rs
    pub fn generate_tile_map(width: usize, height: usize) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                tiles.push(Tile::new(x, y));
            }
        }
        tiles
    }

    pub fn center_location(&self) -> Coordinate {
        Coordinate {
            x: self.x as f32 * TILE_SIDE_LENGTH + TILE_OFFSET,
            y: self.y as f32 * TILE_SIDE_LENGTH + TILE_OFFSET,
        }
    }

    pub fn create_entity(
        self,
        world: &mut World,
        tile_sprite_sheet: Handle<SpriteSheet>,
    ) -> Entity {
        let mut transform = Transform::default();
        let Coordinate {x, y} = self.center_location();
        transform.set_translation_xyz(x, y, 0.0 );

        let sprite_render = SpriteRender {
            sprite_sheet: tile_sprite_sheet.clone(),
            sprite_number: pick_map_sprite_index(self.x, self.y),
        };

        world
            .create_entity()
            .with(self)
            .with(transform)
            .with(sprite_render)
            .build()
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

fn pick_map_sprite_index(x: usize, y: usize) -> usize {
    let mut index = 0;
    if x % 2 != 0 {
        index += 1;
    }
    if y % 2 == 0 {
        index += 2;
    }
    index
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tile_map_generation() {
        let tile_map = Tile::generate_tile_map(10, 20);

        let tile_0 = tile_map.get(4).unwrap();
        assert_eq!(4, tile_0.x);
        assert_eq!(0, tile_0.y);

        let tile_1 = tile_map.get(15 * 10 + 5).unwrap();
        assert_eq!(5, tile_1.x);
        assert_eq!(15, tile_1.y);
    }

    #[test]
    fn test_pick_map_sprite_index() {
        assert_eq!(2, pick_map_sprite_index(0, 0));
        assert_eq!(3, pick_map_sprite_index(1, 0));
        assert_eq!(2, pick_map_sprite_index(2, 0));
        assert_eq!(3, pick_map_sprite_index(3, 0));

        assert_eq!(0, pick_map_sprite_index(0, 1));
        assert_eq!(1, pick_map_sprite_index(1, 1));

        assert_eq!(2, pick_map_sprite_index(0, 2));
        assert_eq!(3, pick_map_sprite_index(1, 2));
    }
}
