use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::components::physics::Physics;

pub struct Resource {
    pub physics: Physics
}

impl Resource {
    pub fn new(width: f32, height: f32) -> Resource {
        Resource {
            physics: Physics::new(width, height)
        }
    }
}

impl Component for Resource {
    type Storage = DenseVecStorage<Self>;
}
