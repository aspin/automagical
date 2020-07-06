use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage, Entities, LazyUpdate};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::core::math::geometry::Point3;
use amethyst::renderer::{Camera, SpriteRender};
use amethyst::window::ScreenDimensions;
use crate::entities::Tile;
use crate::resources::WorldMap;
use crate::components::physics::Coordinate;
use crate::entities::conveyor::{CONVEYOR_Z_INDEX, Conveyor};
use crate::resources::textures::Textures;

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
        ReadExpect<'s, Textures>,
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
            textures,
        ): Self::SystemData) {
        let left_clicked = input.action_is_down("place").unwrap();
        // TODO: if let Some(clicked) = input.action_is_down; clicked { ?
        if left_clicked {
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
                    if tile.occupied() {
                        // TODO: this is fired as long as mouse is held down
                        println!("removing tile!")
                        // let placed_object = tile.placed_object.unwrap();
                        // entities.delete(placed_object);
                        // tile.placed_object.take();
                    } else {
                        println!("placing!");
                        let Coordinate {x, y} = tile.center_location();
                        let mut transform = Transform::default();
                        transform.set_translation_xyz(x, y, CONVEYOR_Z_INDEX);

                        let sprite_render = SpriteRender {
                            sprite_sheet: textures.conveyor_sprite_handle.clone(),
                            sprite_number: 0
                        };

                        let placed_conveyor = entities
                            .build_entity()
                            .with(Conveyor::normal(), &mut conveyors)
                            .with(transform, &mut transforms)
                            .with(sprite_render, &mut sprites)
                            .build();
                        tile.placed_object.replace(placed_conveyor);
                    }
                }
            }
        }


        let right_clicked = input.action_is_down("remove").unwrap();
        if right_clicked {
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
                    if tile.occupied() {
                        println!("trying to remove!");
                        let placed_object = tile.placed_object.unwrap();
                        entities.delete(placed_object);
                        tile.placed_object.take();
                    } else {
                    }
                }
            }
        }
    }
}
