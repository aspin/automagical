use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};
use crate::entities::{Resource, Conveyor};

#[derive(SystemDesc)]
pub struct ConveyorMovementSystem;

impl<'s> System<'s> for ConveyorMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Conveyor>,
        ReadStorage<'s, Resource>
    );

    fn run(&mut self, (mut transforms, conveyors, resources): Self::SystemData) {
        for (conveyor, conveyor_transform) in (&conveyors, &transforms).join() {
            for (resource, resource_transform) in (&resources, &transforms).join() {
                let conveyor_y_min_max = (
                    conveyor_transform.translation().y,
                    conveyor_transform.translation().y
                );
            }
        }
    }
}