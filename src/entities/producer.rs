use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::renderer::{SpriteSheet, SpriteRender};

const PRODUCTION_RATE: u32 = 1;

pub struct Producer {
    production_rate: u32,
}

impl Producer {
    fn log_factory() -> Producer {
        Producer {
            production_rate: PRODUCTION_RATE,
        }
    }

    pub fn create_log_factory(
        world: &mut World,
        x: f32,
        y: f32,
        sprite_sheet: Handle<SpriteSheet>
    ) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.2);

        let sprite_render = SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(Producer::log_factory())
            .with(transform)
            .with(sprite_render)
            .build()
    }
}

impl Component for Producer {
    type Storage = DenseVecStorage<Self>;
}
