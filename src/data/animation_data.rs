use crate::animation::{AnimationState, UnitType};

fn wizard_animation(state: &AnimationState) -> AnimationInfo {
    match state {
        AnimationState::Idle => AnimationInfo::new(7, vec![0.5, 0.1, 0.06, 0.1, 0.1, 0.1], true),
        AnimationState::Attack => {
            AnimationInfo::new(0, vec![0.5, 0.1, 0.3, 0.1, 0.1, 0.1, 0.1], false)
        }
        AnimationState::Move => AnimationInfo::new(14, vec![0.5, 0.1, 0.1, 0.1], false),
    }
}

fn enemy_animation(state: &AnimationState) -> AnimationInfo {
    match state {
        AnimationState::Idle => AnimationInfo::new(7, vec![0.5, 0.1, 0.1, 0.1], true),
        AnimationState::Attack => AnimationInfo::new(0, vec![0.5, 0.1, 0.1, 0.1], false),
        AnimationState::Move => AnimationInfo::new(14, vec![0.5, 0.1, 0.1, 0.1], false),
    }
}

fn arrow_animation(_state: &AnimationState) -> AnimationInfo {
    AnimationInfo::new(0, vec![1.], true)
}

pub fn get_animation_info(unit_type: &UnitType, state: &AnimationState) -> AnimationInfo {
    match unit_type {
        UnitType::Wizard => wizard_animation(state),
        UnitType::Enemy => enemy_animation(state),
        UnitType::Arrow => arrow_animation(state),
    }
}

pub struct AnimationInfo {
    pub sprite_offset: u32,
    pub durations: Vec<f32>,
    pub length: u32,
    pub loop_around: bool,
}

impl AnimationInfo {
    pub fn new(sprite_offset: u32, durations: Vec<f32>, loop_around: bool) -> Self {
        let length = durations.len() as u32;
        AnimationInfo {
            sprite_offset,
            durations,
            loop_around,
            length,
        }
    }
}
