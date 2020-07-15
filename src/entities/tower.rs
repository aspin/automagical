use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Tower {
    pub fire_rate: f32,
    pub capacity: f32,
    pub inventory: f32,
    pub time_since_last_shot: Option<f32>,
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

    pub fn pass_time_between_shots(&mut self, time_delta: f32) {
        if let Some(time_since_last_shot) = self.time_since_last_shot {
            let updated_time = time_since_last_shot - time_delta;
            if updated_time > 0. {
                self.time_since_last_shot.replace(updated_time);
            } else {
                self.time_since_last_shot.take();
            }
        }
    }

    pub fn on_fire(&mut self) {
        self.inventory -= 1.;
        self.time_since_last_shot.replace(self.fire_rate);
    }
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}