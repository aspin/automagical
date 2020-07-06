use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Textures {
    pub character_sprite_handle: Handle<SpriteSheet>,
    pub map_sprite_handle: Handle<SpriteSheet>,
    pub conveyor_sprite_handle: Handle<SpriteSheet>,
    pub resource_sprite_handle: Handle<SpriteSheet>,
}

impl Textures {
    pub fn new(
        character_sprite_handle: Handle<SpriteSheet>,
        map_sprite_handle: Handle<SpriteSheet>,
        conveyor_sprite_handle: Handle<SpriteSheet>,
        resource_sprite_handle: Handle<SpriteSheet>,
    ) -> Textures {
        Textures {
            character_sprite_handle,
            map_sprite_handle,
            conveyor_sprite_handle,
            resource_sprite_handle
        }
    }


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

pub fn load_map_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    load_sprite_sheet(
        world,
        "texture/map_spritesheet.png",
        "texture/map_spritesheet.ron"
    )
}

pub fn load_character_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    load_sprite_sheet(
        world,
        "texture/builder.png",
        "texture/builder_spritesheet.ron"
    )
}

pub fn load_conveyor_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    load_sprite_sheet(
        world,
        "texture/conveyor.png",
        "texture/conveyor_spritesheet.ron"
    )
}

pub fn load_resource_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    load_sprite_sheet(
        world,
        "texture/wood.png",
        "texture/resource_spritesheet.ron"
    )
}
