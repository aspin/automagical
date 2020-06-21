use amethyst::core::{SystemDesc, Transform};
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteExpect, WriteStorage};
use amethyst::ui::UiText;

use crate::colors::{BLACK, WHITE};
use crate::pong::{ARENA_HEIGHT, Ball, BALL_RADIUS, Paddle, Side, SPEED_UP_TEXT_DISAPPEAR_S, SpeedUpText};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, UiText>,
        WriteExpect<'s, SpeedUpText>,
        Read<'s, Time>,
    );

    fn run(&mut self, (transforms, mut balls, paddles, mut ui_text, mut speedup_text, time): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            match speedup_text.disappear_countdown {
                None => {}
                Some(countdown) => {
                    let time_left = countdown - time.delta_seconds();
                    if time_left <= 0.0 {
                        speedup_text.disappear_countdown.take();
                        if let Some(text) = ui_text.get_mut(speedup_text.entity) {
                            text.color = BLACK;
                        }
                    } else {
                            speedup_text.disappear_countdown.replace(time_left);
                    }
                }
            }

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
                    if (paddle.side == Side::Left && *x_velocity < 0.0)
                        || (paddle.side == Side::Right && *x_velocity > 0.0)
                    {
                        *x_velocity *= -1.0;
                        ball.bounces += 1;

                        if ball.bounces % 5 == 0 {

                            if let Some(text) = ui_text.get_mut(speedup_text.entity) {
                                text.color = WHITE;
                                speedup_text.disappear_countdown.replace(SPEED_UP_TEXT_DISAPPEAR_S);
                            }
                            *x_velocity *= 1.2;
                        }
                    }
                }
            }

        }
    }
}
