use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::components::physics::Orientation;

const DEFAULT_BUILDER_SPEED: f32 = 2.;

#[derive(Debug)]
pub struct CoreBuilder {
    pub speed: f32,
    pub orientation: Orientation
}

impl CoreBuilder {
    pub fn new() -> CoreBuilder {
        CoreBuilder {
            speed: DEFAULT_BUILDER_SPEED,
            orientation: Orientation::Down
        }
    }
}

impl Component for CoreBuilder {
    type Storage = DenseVecStorage<Self>;
}
