use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Projectile {}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
