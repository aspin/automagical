use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use crate::entities::CoreBuilder;
use crate::components::physics::Orientation;

#[derive(SystemDesc)]
pub struct BuilderSystem;

impl<'s> System<'s> for BuilderSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, CoreBuilder>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transforms, mut builders, input): Self::SystemData) {
        for (core_builder, transform) in (&mut builders, &mut transforms).join() {
            let delta_x = input.axis_value("horizontal").unwrap();
            let delta_y = input.axis_value("vertical").unwrap();
            let builder_x = transform.translation().x;
            let builder_y = transform.translation().y;

            let new_x = builder_x + delta_x * core_builder.speed;
            let new_y = builder_y + delta_y * core_builder.speed;

            transform.set_translation_x(new_x);
            transform.set_translation_y(new_y);

            // snap to grid after movement is completed
            // if delta_x == 0. && (builder_x - TILE_OFFSET) % TILE_SIDE_LENGTH != 0. {
            //     transform.set_translation_x(
            //         round_to_nearest(
            //             builder_x,
            //             TILE_SIDE_LENGTH,
            //             TILE_OFFSET,
            //             core_builder.orientation.positive_axes().0
            //         )
            //     );
            // }

            // if delta_y == 0. && (builder_y - TILE_OFFSET) % TILE_SIDE_LENGTH != 0. {
            //     transform.set_translation_y(
            //         round_to_nearest(
            //             builder_y,
            //             TILE_SIDE_LENGTH,
            //             TILE_OFFSET,
            //             core_builder.orientation.positive_axes().1
            //         )
            //     );
            // }

            // set orientation after all movement has been computed
            core_builder.orientation = Orientation::from_movement(delta_x, delta_y);
            // println!(
            //     "orientation: {:?}, coordinates: ({}, {})",
            //     core_builder.orientation,
            //     transform.translation().x,
            //     transform.translation().y
            // );
        }
    }
}

fn round_to_nearest(location: f32, factor: f32, offset: f32, round_up: bool) -> f32 {
    let remainder = location % factor;
    if round_up {
        let rounded = location + factor - remainder;
        if factor - remainder > offset {
            rounded - offset
        } else {
            rounded + offset
        }
    } else {
        let rounded = location - remainder;
        if remainder > offset {
            rounded + offset
        } else {
            rounded - offset
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round_to_nearest() {
        assert_eq!(15., round_to_nearest(13.5, 5., 0., true));
        assert_eq!(10., round_to_nearest(13.5, 5., 0., false));
    }

    #[test]
    fn test_round_to_nearest_offset() {
        assert_eq!(25., round_to_nearest(18.5, 10., 5., true));
        assert_eq!(15., round_to_nearest(18.5, 10., 5., false));
        assert_eq!(15., round_to_nearest(13.5, 10., 5., true));
        assert_eq!(5., round_to_nearest(13.5, 10., 5., false));
    }
}
