use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage, Entities, LazyUpdate};
use amethyst::renderer::SpriteRender;

use crate::entities::{Resource, Producer, Tile};
use crate::resources::WorldMap;
use crate::resources::textures::Textures;
use crate::components::physics::{Coordinate, Physics};

#[derive(SystemDesc)]
pub struct ProductionSystem;

impl<'s> System<'s> for ProductionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Producer>,
        ReadExpect<'s, WorldMap>,
        WriteStorage<'s, Resource>,
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, Textures>,
        ReadStorage<'s, Tile>,
        WriteStorage<'s, Physics>
    );

    fn run(
        &mut self,
        (
            entities,
            mut producers,
            world_map,
            mut resources,
            time,
            mut transforms,
            mut sprites,
            textures,
            tiles,
            mut physics,
        ): Self::SystemData
    ) {
        let mut resources_to_generate: Vec<(Resource, Transform, SpriteRender, Physics)> = Vec::new();

        for (producer, transform) in (&mut producers, &transforms).join() {
            producer.update_inventory(time.delta_seconds());

            if producer.inventory >= 1. {
                producer.inventory -= 1.;

                let translation = transform.translation();
                let (x, y) = world_map.coordinate_to_x_y(
                    translation.x, translation.y
                );
                // TODO: fix me
                let generation_tile = world_map.get_tile_entity(x + 1, y + 1).unwrap();
                let Coordinate {x, y} = tiles.get(*generation_tile).unwrap().center_location();
                let mut transform = Transform::default();
                transform.set_translation_xyz(x, y, 0.2);

                let sprite_render = SpriteRender {
                    sprite_sheet: textures.resource_sprite_handle.clone(),
                    sprite_number: 0
                };

                resources_to_generate.push(
                    (Resource::new(), transform, sprite_render, Physics::new(4., 4.))
                );
            }
        }

        for (resource, transform, sprite_render, physic) in resources_to_generate {
            entities
                .build_entity()
                .with(resource, &mut resources)
                .with(transform, &mut transforms)
                .with(sprite_render, &mut sprites)
                .with(physic, &mut physics)
                .build();
        }
    }
}