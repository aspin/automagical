use amethyst::ecs::Entity;

/// TODO: world map struct will not currently support infinitely expanding maps
/// for now, the implementation will be of a fixed size

pub struct WorldMap {
    tiles: Vec<Entity>,
    width: usize,
    height: usize,
}

impl WorldMap {
    pub fn new(tiles: Vec<Entity>, width: usize, height: usize) -> WorldMap {
        WorldMap { tiles, width, height }
    }

    pub fn get_tile_entity(&self, x: usize, y: usize) -> Option<&Entity> {
        let index = y * self.width + x;
        self.tiles.get(index)
    }
}

