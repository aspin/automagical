use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use crate::entities::CoreBuilder;
use crate::resources::WorldMap;
use crate::entities::Tile;

const CAMERA_WIDTH: f32 = 100.;
const CAMERA_HEIGHT: f32 = 100.;

const WORLD_WIDTH: usize = 200;
const WORLD_HEIGHT: usize = 200;

#[derive(Default)]
pub struct Automagical {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Automagical {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialize_camera(world);
        initialize_world_map(world, WORLD_WIDTH, WORLD_HEIGHT);
        initialize_builder(world, self.sprite_sheet_handle.clone().unwrap());
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

fn initialize_world_map(world: &mut World, width: usize, height: usize) {
    world.register::<Tile>();

    let tiles: Vec<Tile> = Tile::generate_tile_map(width, height);
    let mut entities: Vec<Entity> = Vec::with_capacity(width * height);
    for tile in tiles {
        let entity = world
            .create_entity()
            .with(tile)
            .build();
        entities.push(entity);
    }
    world.insert(WorldMap::new(entities, width, height));
}

fn initialize_builder(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(CoreBuilder::new())
        .with(transform)
        .with(sprite_render)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
