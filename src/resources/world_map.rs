use amethyst::ecs::Entity;

/// TODO: world map struct will not currently support infinitely expanding maps
/// for now, the implementation will be of a fixed size

pub struct WorldMap {
    tiles: Vec<Entity>,
    width: usize,  // number of tiles
    height: usize,
    world_width: f32,  // number of pixels
    world_height: f32,
}

impl WorldMap {
    pub fn new(
        tiles: Vec<Entity>,
        width: usize,
        height: usize,
        world_width: f32,
        world_height: f32,
    ) -> WorldMap {
        WorldMap { tiles, width, height, world_width, world_height }
    }

    pub fn get_tile_entity(&self, x: usize, y: usize) -> Option<&Entity> {
        let index = y * self.width + x;
        self.tiles.get(index)
    }

    pub fn coordinate_to_tile(&self, x_coordinate: f32, y_coordinate: f32) -> Option<&Entity> {
        let x = (x_coordinate / self.tile_width()) as usize;
        let y = (y_coordinate / self.tile_height()) as usize;
        self.get_tile_entity(x, y)
    }

    fn tile_width(&self) -> f32 {
        self.world_width / self.width as f32
    }

    fn tile_height(&self) -> f32 {
        self.world_height / self.height as f32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use amethyst::ecs::world::EntitiesRes;

    #[test]
    fn test_get_tile() {
        // world of size 150 x 150
        let side_length: usize = 10;
        let side_dimension = 15.;
        let entities_res = EntitiesRes::default();

        let mut entities: Vec<Entity> = Vec::with_capacity(100);
        for i in 0..side_length {
            for j in 0..side_length {
                let entity = entities_res.create();
                entities.push(entity);
            }
        }

        let world_map = WorldMap::new(
            entities.clone(),
            side_length,
            side_length,
            side_length as f32 * side_dimension,
            side_length as f32 * side_dimension
        );

        let matching_tile_entity = world_map.coordinate_to_tile(
            78., 41.
        ).unwrap();

        // entities are ordered opposite the tile map
        let expected_entity = entities.get(10 * 2 + 5).unwrap();

        assert_eq!(matching_tile_entity, expected_entity);
    }
}

