use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::components::physics::Physics;

const NORMAL_CONVEYOR_SPEED: f32 = 8.;
const CONVEYOR_WIDTH: f32 = 16.;
const CONVEYOR_HEIGHT: f32 = 16.;

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

    pub fn normal() -> Conveyor {
        Self::new(NORMAL_CONVEYOR_SPEED, CONVEYOR_WIDTH, CONVEYOR_HEIGHT)
    }
}

impl Component for Conveyor {
    type Storage = DenseVecStorage<Self>;
}
