use amethyst::core::timing::Time;
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, System, SystemData, WriteStorage, Write};
use amethyst::ui::UiText;

use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH, Ball, BALL_RADIUS, Paddle, Score, Side, RESPAWN_BALL_INTERVAL, ScoreText};

#[derive(SystemDesc)]
pub struct RoundSystem;

impl RoundSystem {
    fn reset_ball_transform(&mut self, ball: &mut Ball, transform: &mut Transform, elapsed_time: f32) {
        match ball.reset_time_countdown {
            None => {ball.reset_time_countdown.replace(RESPAWN_BALL_INTERVAL);},
            Some(countdown) => {
                let time_left = countdown - elapsed_time;
                if time_left <= 0.0 {
                    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
                    ball.velocity.0 *= -1.0;
                    ball.reset_time_countdown.take();
                } else {
                    ball.reset_time_countdown.replace(time_left);
                }
            }
        };
    }
}

impl<'s> System<'s> for RoundSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        Write<'s, Score>,
        Read<'s, Time>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (mut transforms, mut balls, mut score, time, mut ui_text, score_text): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let scored = if transform.translation().x < 0.0 {
                // score on left
                score.right += 1;

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = score.right.to_string()
                }
                println!("Score: P1 {} P2 {}", score.left, score.right);
                true
            } else if transform.translation().x > ARENA_WIDTH {
                // score on right
                score.left += 1;
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = score.left.to_string()
                }
                println!("Score: P1 {} P2 {}", score.left, score.right);
                true
            } else {
                false
            };

            if scored {
                self.reset_ball_transform(ball, transform, time.delta_seconds());
            }
        }
    }

}
