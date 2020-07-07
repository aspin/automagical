use crate::entities::Tile;
use crate::entities::{CoreBuilder, Resource};
use crate::resources::textures::{load_character_sprite_sheet, load_conveyor_sprite_sheet, load_map_sprite_sheet, load_resource_sprite_sheet, Textures, load_tower_sprite_sheet, load_projectile_sprite_sheet};
use crate::resources::WorldMap;
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{Camera, SpriteSheet},
};

const CAMERA_WIDTH: f32 = 320.;
const CAMERA_HEIGHT: f32 = 320.;

const TILE_COUNT_X: usize = 20;
const TILE_COUNT_Y: usize = 20;

#[derive(Default)]
pub struct Automagical {}

impl SimpleState for Automagical {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let map_sprite_handle = load_map_sprite_sheet(world);
        let character_sprite_handle = load_character_sprite_sheet(world);
        let conveyor_sprite_handle = load_conveyor_sprite_sheet(world);
        let resource_sprite_handle = load_resource_sprite_sheet(world);
        let tower_sprite_handle = load_tower_sprite_sheet(world);
        let projectile_sprite_handle = load_projectile_sprite_sheet(world);

        initialize_camera(world);
        initialize_world_map(
            world,
            map_sprite_handle.clone(),
            TILE_COUNT_X,
            TILE_COUNT_Y,
        );
        CoreBuilder::create_entity(
            world,
            CAMERA_WIDTH * 0.5,
            CAMERA_HEIGHT * 0.5,
            character_sprite_handle.clone(),
        );
        world.insert(Textures::new(
            character_sprite_handle,
            map_sprite_handle,
            conveyor_sprite_handle,
            resource_sprite_handle,
            tower_sprite_handle,
            projectile_sprite_handle
        ));
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_world_map(
    world: &mut World,
    tile_sprite_sheet: Handle<SpriteSheet>,
    tile_count_x: usize,
    tile_count_y: usize,
) {
    // TODO: remove this line once a system uses it
    world.register::<Resource>();

    let tiles: Vec<Tile> = Tile::generate_tile_map(tile_count_x, tile_count_y);
    let mut entities: Vec<Entity> = Vec::with_capacity(tile_count_x * tile_count_y);
    for tile in tiles {
        let entity = tile.create_entity(world, tile_sprite_sheet.clone());
        entities.push(entity);
    }

    world.insert(WorldMap::new(
        entities,
        tile_count_x,
        tile_count_y,
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
    ));
}
