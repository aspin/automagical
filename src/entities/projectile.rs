use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::components::physics::{Physics, Orientation};

const ARROW_LIFESPAN: f32 = 3.;

pub struct Projectile {
    pub lifespan: f32
}

impl Projectile {
    pub fn arrow() -> Projectile {
        Projectile {
            lifespan: ARROW_LIFESPAN
        }
    }

    pub fn arrow_physics() -> Physics {
        Physics {
            velocity: (100., Orientation::Right),
            width: 8.0,
            height: 4.0
        }
    }
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
