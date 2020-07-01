use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::entities::Orientation;

pub struct Conveyor {
    pub orientation: Orientation
}

impl Conveyor {
    pub fn new() -> Conveyor {
        Conveyor {
            orientation: Orientation::Up
        }
    }
}

impl Component for Conveyor {
    type Storage = DenseVecStorage<Self>;
}