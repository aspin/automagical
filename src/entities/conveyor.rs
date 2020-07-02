use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::components::physics::Physics;

pub struct Conveyor {
    pub physics: Physics
}

impl Conveyor {
    pub fn new(width: f32, height: f32) -> Conveyor {
        Conveyor {
            physics: Physics::new(width, height)
        }
    }
}

impl Component for Conveyor {
    type Storage = DenseVecStorage<Self>;
}