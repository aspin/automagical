use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Entity, ReadStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use crate::entities::{CoreBuilder, Conveyor, Resource};
use crate::resources::WorldMap;
use crate::entities::Tile;
use crate::utils::constants::{TILE_SIDE_LENGTH, TILE_OFFSET};

const CAMERA_WIDTH: f32 = 320.;
const CAMERA_HEIGHT: f32 = 320.;

const TILE_COUNT_X: usize = 20;
const TILE_COUNT_Y: usize = 20;

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
        CoreBuilder::create_entity(
            world,
            CAMERA_WIDTH * 0.5,
            CAMERA_HEIGHT * 0.5,
            self.character_sprite_handle.clone().unwrap()
        );
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
    world.register::<Resource>();

    let tiles: Vec<Tile> = Tile::generate_tile_map(tile_count_x, tile_count_y);
    let mut entities: Vec<Entity> = Vec::with_capacity(tile_count_x * tile_count_y);
    for tile in tiles {
        // let x = tile.x;
        // let y = tile.y;
        // let x_location = tile.x as f32 * TILE_SIDE_LENGTH + TILE_OFFSET;
        // let y_location = tile.y as f32 * TILE_SIDE_LENGTH + TILE_OFFSET;

        let entity = tile.create_entity(world, tile_sprite_sheet.clone());

        // if x == 4 {
        //     Conveyor::create_entity(
        //         world,
        //         y as f32 * 5.,
        //         x_location,
        //         y_location,
        //         conveyor_sprite_sheet.clone()
        //     );
        //     if y == 1 {
        //         for i in 0..2 {
        //             Resource::create_entity(
        //                 world,
        //                 x_location,
        //                 y_location + (i as f32) * 4.,
        //                 resource_sprite_sheet.clone()
        //             );
        //         }
        //     }
        // }

        entities.push(entity);
    }
    world.insert(WorldMap::new(
        entities,
        tile_count_x,
        tile_count_y,
        CAMERA_WIDTH,
        CAMERA_HEIGHT
    ));
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

