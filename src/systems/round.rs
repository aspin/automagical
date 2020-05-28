use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH, Ball, BALL_RADIUS, Paddle, Score, Side};

#[derive(SystemDesc, Default)]
pub struct RoundSystem {
    scored_time: Option<f32>,
}
//
// impl RoundSystem {
//     fn reset_ball_transform(&mut self, ball: &mut Ball, transform: &mut Transform) {
//         transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
//         ball.velocity.0 *= -1.0;
//     }
// }

impl<'s> System<'s> for RoundSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Score>
    );

    fn run(&mut self, (mut transforms, mut balls, mut scores): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let scored = if transform.translation().x < 0.0 {
                // score on left
                for score in (&mut scores).join() {
                    score.left += 1;
                    println!("Score: P1 {} P2 {}", score.left, score.right);
                }
                true
            } else if transform.translation().x > ARENA_WIDTH {
                // score on right
                for score in (&mut scores).join() {
                    score.right += 1;
                    println!("Score: P1 {} P2 {}", score.left, score.right);
                }
                true
            } else {
                false
            };

            if scored {
                self.reset_ball_transform(ball, transform);
            }
        }
    }

}
