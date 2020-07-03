use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::renderer::{SpriteSheet, SpriteRender};
use crate::utils::constants::{TILE_SIDE_LENGTH, TILE_OFFSET};

pub struct Tile {
    pub x: usize,
    pub y: usize,
}

impl Tile {
    pub fn new(x: usize, y: usize) -> Tile {
        Tile {
            x, y
        }
    }

    pub fn generate_tile_map(width: usize, height: usize) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                tiles.push(Tile::new(x, y));
            }
        }
        tiles
    }

    pub fn create_entity(
        self,
        world: &mut World,
        tile_sprite_sheet: Handle<SpriteSheet>,
    ) -> Entity {
        let mut transform = Transform::default();
        let x_location = self.x as f32 * TILE_SIDE_LENGTH + TILE_OFFSET;
        let y_location = self.y as f32 * TILE_SIDE_LENGTH + TILE_OFFSET;
        transform.set_translation_xyz(x_location, y_location, 0.0);

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
