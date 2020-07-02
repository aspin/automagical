use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use crate::entities::{CoreBuilder, Conveyor, Resource};
use crate::resources::WorldMap;
use crate::entities::Tile;
use crate::utils::constants::{TILE_SIDE_LENGTH, TILE_OFFSET};

const CAMERA_WIDTH: f32 = 160.;
const CAMERA_HEIGHT: f32 = 160.;

const TILE_COUNT_X: usize = 10;
const TILE_COUNT_Y: usize = 10;

#[derive(Default)]
pub struct Automagical {
    character_sprite_handle: Option<Handle<SpriteSheet>>,
    map_sprite_handle: Option<Handle<SpriteSheet>>,
    conveyor_sprite_handle: Option<Handle<SpriteSheet>>,
    resource_sprite_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Automagical {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.map_sprite_handle.replace(
            load_sprite_sheet(
                world,
                "texture/map_spritesheet.png",
                "texture/map_spritesheet.ron"
            )
        );
        self.character_sprite_handle.replace(
            load_sprite_sheet(
                world,
                "texture/builder.png",
                "texture/builder_spritesheet.ron"
            )
        );
        self.conveyor_sprite_handle.replace(
            load_sprite_sheet(
                world,
                "texture/conveyor.png",
                "texture/conveyor_spritesheet.ron"
            )
        );
        self.resource_sprite_handle.replace(
            load_sprite_sheet(
                world,
                "texture/wood.png",
                "texture/resource_spritesheet.ron"
            )
        );

        initialize_camera(world);
        initialize_world_map(
            world,
            self.map_sprite_handle.clone().unwrap(),
            TILE_COUNT_X,
            TILE_COUNT_Y,
            self.conveyor_sprite_handle.clone().unwrap(),
            self.resource_sprite_handle.clone().unwrap(),
        );
        initialize_builder(world, self.character_sprite_handle.clone().unwrap());
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
    conveyor_sprite_sheet: Handle<SpriteSheet>,
    resource_sprite_sheet: Handle<SpriteSheet>,
) {
    // TODO: remove this line once a system uses it
    world.register::<Tile>();

    let tiles: Vec<Tile> = Tile::generate_tile_map(tile_count_x, tile_count_y);
    let mut entities: Vec<Entity> = Vec::with_capacity(tile_count_x * tile_count_y);
    for tile in tiles {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            tile.x as f32 * TILE_SIDE_LENGTH + TILE_OFFSET,
            tile.y as f32 * TILE_SIDE_LENGTH + TILE_OFFSET,
            0.0
        );

        let sprite_render = SpriteRender {
            sprite_sheet: tile_sprite_sheet.clone(),
            sprite_number: pick_map_sprite_index(tile.x, tile.y),
        };

        if tile.x == 4 {
            let mut conveyor_transform = transform.clone();
            conveyor_transform.set_translation_z(0.1);
            let sprite_render = SpriteRender {
                sprite_sheet: conveyor_sprite_sheet.clone(),
                sprite_number: 0
            };
            world
                .create_entity()
                .with(Conveyor::new())
                .with(conveyor_transform)
                .with(sprite_render)
                .build();

            if tile.y == 1 {
                for i in 0..2 {
                    let mut resource_transform = transform.clone();
                    resource_transform.set_translation_z(0.2);
                    resource_transform.set_translation_y(
                        resource_transform.translation().y + (i as f32) * 4.
                    );
                    let sprite_render = SpriteRender {
                        sprite_sheet: resource_sprite_sheet.clone(),
                        sprite_number: 0
                    };
                    world
                        .create_entity()
                        .with(Resource::new(4., 4.))
                        .with(resource_transform)
                        .with(sprite_render)
                        .build();
                }
            }
        }

        let entity = world
            .create_entity()
            .with(tile)
            .with(transform)
            .with(sprite_render)
            .build();

        entities.push(entity);
    }
    world.insert(WorldMap::new(entities, tile_count_x, tile_count_y));
}

fn initialize_builder(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 0.5);

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

fn load_sprite_sheet(world: &mut World, texture_file: &str, sprite_file: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            texture_file,
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        sprite_file,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn pick_map_sprite_index(x: usize, y: usize) -> usize {
    let mut index = 0;
    if x % 2 != 0 {
        index += 1;
    }
    if y % 2 != 0 {
        index += 2;
    }
    index
}
