#[derive(PartialEq)]
pub enum AnimationState {
    Idle,
    Move,
    Attack,
}

pub enum UnitType {
    Wizard,
    Enemy,
}
