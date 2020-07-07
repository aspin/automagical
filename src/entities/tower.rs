use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Tower {
    pub fire_rate: f32,
    pub capacity: f32,
    pub inventory: f32,
    pub time_since_last_shot: Option<f32>,
    // ammo type
}

impl Tower {
    pub fn arrow_tower() -> Tower {
        Tower {
            fire_rate: 0.5,
            capacity: 100.,
            inventory: 10.,
            time_since_last_shot: Option::None,
        }
    }
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}