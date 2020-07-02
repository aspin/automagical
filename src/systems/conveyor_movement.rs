use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};
use crate::entities::{Resource, Conveyor};
use crate::components::physics::Orientation;

#[derive(SystemDesc)]
pub struct ConveyorMovementSystem;

impl<'s> System<'s> for ConveyorMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Conveyor>,
        WriteStorage<'s, Resource>
    );

    fn run(&mut self, (mut transforms, conveyors, mut resources): Self::SystemData) {
        for (resource, resource_transform) in (&mut resources, &transforms).join() {
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
                resource.physics.velocity = (speed, orientation);
            } else {
                resource.physics.velocity.0 = 0.;
            }
        }
    }
}