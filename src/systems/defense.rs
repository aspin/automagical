use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage, Entities};
use amethyst::renderer::SpriteRender;

use crate::entities::tower::Tower;
use crate::entities::{Projectile, Tile};
use crate::resources::WorldMap;
use crate::components::physics::{Coordinate, Physics, Orientation};
use crate::resources::textures::Textures;
use crate::utils::constants::PROJECTILE_Z_INDEX;


#[derive(SystemDesc)]
pub struct DefenseSystem;

impl<'s> System<'s> for DefenseSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tower>,
        WriteStorage<'s, Projectile>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Physics>,
        Read<'s, Time>,
        ReadExpect<'s, WorldMap>,
        ReadStorage<'s, Tile>,
        ReadExpect<'s, Textures>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut towers,
            mut projectiles,
            mut sprites,
            mut physics,
            time,
            world_map,
            tiles,
            textures
        ): Self::SystemData
    ) {
        let mut projectiles_to_generate: Vec<(Projectile, Transform, SpriteRender, Physics)> = Vec::new();

        for (tower, transform) in (&mut towers, &transforms).join() {
            if let Some(_time_since_last_shot) = tower.time_since_last_shot {
                tower.pass_time_between_shots(time.delta_seconds());
            } else if tower.inventory > 0. {
                tower.on_fire();

                if let Some(generation_transform) = world_map.coordinate_to_tile_transform(
                    transform, &tiles, PROJECTILE_Z_INDEX
                ) {
                    let sprite_render = SpriteRender {
                        sprite_sheet: textures.projectile_sprite_handle.clone(),
                        sprite_number: 0
                    };

                    projectiles_to_generate.push(
                        (
                            Projectile::arrow(),
                            generation_transform,
                            sprite_render,
                            Projectile::arrow_physics()
                        )
                    );
                }
            }
        }

        for (projectile, transform, sprite, physic) in projectiles_to_generate {
            entities
                .build_entity()
                .with(projectile, &mut projectiles)
                .with(transform, &mut transforms)
                .with(sprite, &mut sprites)
                .with(physic, &mut physics)
                .build();
        }
    }
}
