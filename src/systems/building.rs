use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage, Entities, LazyUpdate};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::core::math::geometry::Point3;
use amethyst::renderer::{Camera, SpriteRender, Texture};
use amethyst::window::ScreenDimensions;
use crate::entities::{Tile, Conveyor};
use crate::resources::WorldMap;
use crate::components::physics::Coordinate;

const CONVEYOR_WIDTH: f32 = 16.;
const CONVEYOR_HEIGHT: f32 = 16.;
const CONVEYOR_Z_INDEX: f32 = 0.1;

#[derive(SystemDesc)]
pub struct BuildingSystem;

impl<'s> System<'s> for BuildingSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        WriteStorage<'s, Tile>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, WorldMap>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, Conveyor>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            entities,
            updater,
            mut tiles,
            input,
            world_map,
            cameras,
            mut transforms,
            screen_dimensions,
            mut conveyors,
            mut sprites,
        ): Self::SystemData) {
        let clicked = input.action_is_down("place").unwrap();
        // TODO: if let Some(clicked) = input.action_is_down; clicked { ?
        if clicked {
            if let Some((mouse_x, mouse_y)) = input.mouse_position() {
                if let Some((camera, transform)) = (&cameras, &transforms).join().next() {
                    let mouse_point = Point3::new(mouse_x, mouse_y, 0.0);
                    let world_point = camera
                        .projection()
                        .screen_to_world_point(
                            mouse_point,
                            screen_dimensions.diagonal(),
                            transform
                        );

                    // TODO: this should probably be safer?
                    let tile_entity = world_map.coordinate_to_tile(
                        world_point.x, world_point.y
                    ).unwrap();
                    let tile = tiles.get_mut(*tile_entity).unwrap();
                    if tile.occupied {

                    } else {
                        tile.occupied = true;

                        let Coordinate {x, y} = tile.center_location();
                        let mut transform = Transform::default();
                        transform.set_translation_xyz(x, y, CONVEYOR_Z_INDEX);

                        let sprite_render = SpriteRender {
                            sprite_sheet: world_map.conveyor_sprite_handle.clone(),
                            sprite_number: 0
                        };
                        let speed = 5.;

                        entities.build_entity()
                            .with(
                                Conveyor::new(
                                    speed, CONVEYOR_WIDTH, CONVEYOR_HEIGHT
                                ),
                                &mut conveyors
                            )
                            .with(transform, &mut transforms)
                            .with(sprite_render, &mut sprites)
                            .build();
                    }
                }
            }
        }
    }
}
