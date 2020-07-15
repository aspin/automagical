use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::components::physics::{Physics, Orientation};

const ARROW_LIFESPAN: f32 = 1.;

pub struct Projectile {
    pub ttl: f32,
    pub lifespan: f32
}

impl Projectile {
    pub fn arrow() -> Projectile {
        Projectile {
            lifespan: ARROW_LIFESPAN,
            ttl: ARROW_LIFESPAN
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
