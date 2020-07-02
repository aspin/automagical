use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Conveyor {
}

impl Conveyor {
    pub fn new() -> Conveyor {
        Conveyor {
        }
    }
}

impl Component for Conveyor {
    type Storage = DenseVecStorage<Self>;
}