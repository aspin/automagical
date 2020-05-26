use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};
use crate::pong::{Ball, BALL_RADIUS, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Ball>
    );

    fn run(&mut self, (transforms, mut balls): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            if transform.translation().y <= BALL_RADIUS * 0.5 || transform.translation().y >= ARENA_HEIGHT - BALL_RADIUS * 0.5 {
                ball.velocity[1] *= -1.0;
            }
            if transform.translation().x <= BALL_RADIUS * 0.5 || transform.translation().x >= ARENA_WIDTH - BALL_RADIUS * 0.5 {
                ball.velocity[0] *= -1.0;
            }
        }
    }
}
