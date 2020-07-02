use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::renderer::{SpriteSheet, SpriteRender};
use crate::components::physics::{Orientation, Physics};

const DEFAULT_BUILDER_SPEED: f32 = 2.;
const BUILDER_Z_INDEX: f32 = 0.5;

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

    pub fn create_entity(
        world: &mut World,
        x: f32,
        y: f32,
        builder_sprite_sheet: Handle<SpriteSheet>
    ) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, BUILDER_Z_INDEX);

        let sprite_render = SpriteRender {
            sprite_sheet: builder_sprite_sheet,
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(CoreBuilder::new())
            .with(transform)
            .with(sprite_render)
            .with(Physics::new(16., 16.))
            .build()
    }
}

impl Component for CoreBuilder {
    type Storage = DenseVecStorage<Self>;
}
