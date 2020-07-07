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
            if let Some(time_since_last_shot) = tower.time_since_last_shot {
                let updated_time = time_since_last_shot - time.delta_seconds();
                if updated_time > 0. {
                    tower.time_since_last_shot.replace(updated_time);
                } else {
                    tower.time_since_last_shot.take();
                }
            } else if tower.inventory > 0. {
                tower.inventory -= 1.;
                tower.time_since_last_shot.replace(tower.fire_rate);

                let tower_center = transform.translation();
                let (x, y) = world_map.coordinate_to_x_y(
                    tower_center.x, tower_center.y
                );
                let generation_tile = world_map.get_tile_entity(x, y).unwrap();
                let Coordinate {x, y} = tiles.get(*generation_tile).unwrap().center_location();
                let mut transform = Transform::default();
                transform.set_translation_xyz(x, y, 0.3);

                let sprite_render = SpriteRender {
                    sprite_sheet: textures.projectile_sprite_handle.clone(),
                    sprite_number: 0
                };

                projectiles_to_generate.push(
                    (
                        Projectile{},
                        transform,
                        sprite_render,
                        Physics {
                            velocity: (100., Orientation::Right),
                            width: 8.0,
                            height: 4.0
                        }
                    )
                );
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
