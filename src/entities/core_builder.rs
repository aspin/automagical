use amethyst::ecs::prelude::{Component, DenseVecStorage};

const DEFAULT_BUILDER_SPEED: f32 = 1.2;

#[derive(Debug)]
pub enum Orientation {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    None
}

impl Orientation {
    /// Indicates whether direction is positive along x, y axis.
    pub fn positive_axes(&self) -> (bool, bool) {
        match *self {
            Orientation::Left => (false, false),
            Orientation::Right => (true, false),
            Orientation::Down => (false, false),
            Orientation::Up => (false, true),
            Orientation::UpLeft => (false, true),
            Orientation::UpRight => (true, true),
            Orientation::DownLeft => (false, false),
            Orientation::DownRight => (true, false),
            Orientation::None => (false, false),
        }
    }

    pub fn from_movement(x: f32, y: f32) -> Orientation {
        if x < 0.0 && y == 0.0 {
            Orientation::Left
        } else if x > 0.0 && y == 0.0 {
            Orientation::Right
        } else if x == 0.0 && y > 0.0 {
            Orientation::Up
        } else if x == 0.0 && y < 0.0 {
            Orientation::Down
        } else if x < 0.0 && y < 0.0 {
            Orientation::DownLeft
        } else if x > 0.0 && y < 0.0 {
            Orientation::DownRight
        } else if x < 0.0 && y > 0.0 {
            Orientation::UpLeft
        } else if x > 0.0 && y > 0.0 {
            Orientation::UpRight
        } else {
            Orientation::None
        }
    }
}

pub struct CoreBuilder {
    pub speed: f32,
    pub orientation: Orientation
}

impl CoreBuilder {
    pub fn new() -> CoreBuilder {
        CoreBuilder {
            speed: DEFAULT_BUILDER_SPEED,
            orientation: Orientation::Down
        }
    }
}

impl Component for CoreBuilder {
    type Storage = DenseVecStorage<Self>;
}
