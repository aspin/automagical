use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH, Ball, BALL_RADIUS, Paddle, Side};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
    );

    fn run(&mut self, (transforms, mut balls, paddles): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let (ref mut x_velocity, ref mut y_velocity) = ball.velocity;
            // top bottom collisions
            if (transform.translation().y <= BALL_RADIUS * 0.5 && *y_velocity < 0.0)
                || (transform.translation().y >= ARENA_HEIGHT - BALL_RADIUS * 0.5
                && *y_velocity > 0.0)
            {
                *y_velocity *= -1.0;
            }

            // paddle collisions
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                if paddle.ball_collides(paddle_transform, transform) {
                    if paddle.side == Side::Left && *x_velocity < 0.0 {
                        *x_velocity *= -1.0;
                    }
                    if paddle.side == Side::Right && *x_velocity > 0.0 {
                        *x_velocity *= -1.0;
                    }
                }
            }
        }
    }
}


