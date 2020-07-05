use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::ecs::world::EntityResBuilder;
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
}

impl Component for Conveyor {
    type Storage = DenseVecStorage<Self>;
}
