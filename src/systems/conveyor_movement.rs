use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};
use crate::components::physics::{Orientation, Physics};
use crate::entities::conveyor::Conveyor;

#[derive(SystemDesc)]
pub struct ConveyorMovementSystem;

impl<'s> System<'s> for ConveyorMovementSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Conveyor>,
        WriteStorage<'s, Physics>
    );

    fn run(&mut self, (transforms, conveyors, mut physics): Self::SystemData) {
        for (physic, resource_transform) in (&mut physics, &transforms).join() {
            let mut new_physics: Option<(f32, Orientation)> = Option::None;
            for (conveyor, conveyor_transform) in (&conveyors, &transforms).join() {
                if conveyor.physics.intersects(
                    conveyor_transform, resource_transform
                ) {
                    new_physics.replace(
                        (conveyor.speed, conveyor.physics.velocity.1.clone())
                    );
                }
            }

            if let Some((speed, orientation)) = new_physics {
                physic.velocity = (speed, orientation);
            } else {
                physic.velocity.0 = 0.;
            }
        }
    }
}