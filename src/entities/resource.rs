use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::{World, WorldExt, Builder};
use amethyst::renderer::{SpriteSheet, SpriteRender};
use crate::components::physics::Physics;

const RESOURCE_Z_INDEX: f32 = 0.2;

pub struct Resource {
}

impl Resource {
    pub fn create_entity(
        world: &mut World,
        x: f32,
        y: f32,
        resource_sprite_sheet: Handle<SpriteSheet>
    ) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, RESOURCE_Z_INDEX);

        let sprite_render = SpriteRender {
            sprite_sheet: resource_sprite_sheet,
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(Resource::new())
            .with(transform)
            .with(sprite_render)
            .with(Physics::new(4., 4.))
            .build()
    }
}

impl Component for Resource {
    type Storage = DenseVecStorage<Self>;
}
