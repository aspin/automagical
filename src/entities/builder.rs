use amethyst::ecs::prelude::{Component, DenseVecStorage};

const DEFAULT_BUILDER_SPEED: f32 = 1.2;

pub struct CoreBuilder {
    pub speed: f32
}

impl CoreBuilder {
    pub fn new() -> CoreBuilder {
        CoreBuilder {
            speed: DEFAULT_BUILDER_SPEED
        }
    }
}

impl Component for CoreBuilder {
    type Storage = DenseVecStorage<Self>;
}
