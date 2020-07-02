use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::utils::color::{Color, BLACK};

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
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
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
}
