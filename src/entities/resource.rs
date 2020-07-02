use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Resource {}

impl Resource {
    pub fn new() -> Resource {
        Resource {}
    }
}

impl Component for Resource {
    type Storage = DenseVecStorage<Self>;
}
