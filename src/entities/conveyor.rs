use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::renderer::{SpriteSheet, SpriteRender};
use crate::components::physics::Physics;

const CONVEYOR_WIDTH: f32 = 16.;
const CONVEYOR_HEIGHT: f32 = 16.;
const CONVEYOR_Z_INDEX: f32 = 0.1;

pub struct Conveyor {
    pub speed: f32,
    pub physics: Physics
}

impl Conveyor {
    pub fn new(speed: f32, width: f32, height: f32) -> Conveyor {
        Conveyor {
            speed,
            physics: Physics::new(width, height)
        }
    }

    pub fn create_entity(
        world: &mut World,
        speed: f32,
        x: f32,
        y: f32,
        conveyor_sprite_sheet: Handle<SpriteSheet>
    ) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, CONVEYOR_Z_INDEX);

        let sprite_render = SpriteRender {
            sprite_sheet: conveyor_sprite_sheet.clone(),
            sprite_number: 0
        };

        world
            .create_entity()
            .with(Conveyor::new(speed, CONVEYOR_WIDTH, CONVEYOR_HEIGHT))
            .with(transform)
            .with(sprite_render)
            .build()
    }
}

impl Component for Conveyor {
    type Storage = DenseVecStorage<Self>;
}
