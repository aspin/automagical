use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use crate::components::physics::Physics;

pub struct Projectile {

}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
