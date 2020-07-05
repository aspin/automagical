use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage, Entities, LazyUpdate};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::core::math::geometry::Point3;
use amethyst::renderer::Camera;
use amethyst::window::ScreenDimensions;
use crate::entities::Tile;
use crate::resources::WorldMap;

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
        ReadStorage<'s, Transform>,
        ReadExpect<'s, ScreenDimensions>
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
            transforms,
            screen_dimensions
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
                    println!("{:?}", world_point);
                }
            }
        }
    }
}
