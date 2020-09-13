use bevy::prelude::*;


const ANIMATION_SPEED: f32 = 0.5;

pub struct Builder {
    pub name: String,
    pub state: BuilderState,
    pub animation_index: u32,
}

#[derive(PartialEq)]
pub enum BuilderState {
    Idle,
    Move,
    Attack,
}

impl Builder {
    pub fn new(name: &str) -> Builder {
        Builder {
            name: String::from(name),
            state: BuilderState::Idle,
            animation_index: 0,
        }
    }
}

pub fn animate(
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Builder)>
) {
    for (mut timer, mut sprite, mut builder) in &mut query.iter() {
        if timer.finished {
            let (mut offset, mut duration, length, loop_around) = match builder.state {
                BuilderState::Idle => (7, vec![0.5, 0.1, 0.06, 0.1, 0.1, 0.1], 6, true),
                BuilderState::Attack => (0, vec![0.5, 0.1, 0.3, 0.1, 0.1, 0.1, 0.1], 7, false),
                BuilderState::Move => (14, vec![0.5, 0.1, 0.1, 0.1], 4, false),
            };

            let next_index = builder.animation_index + 1;
            if next_index >= length {
                if loop_around {
                    builder.animation_index = next_index % length
                } else {
                    builder.animation_index = 0;
                    builder.state = BuilderState::Idle;
                    offset = 7;
                    duration = vec![0.5, 0.1, 0.06, 0.1, 0.1, 0.1];
                }
            } else {
                builder.animation_index = next_index;
            }

            sprite.index = offset + builder.animation_index;

            timer.reset();
            timer.duration = duration[builder.animation_index as usize] * ANIMATION_SPEED;
        }
    }
}
