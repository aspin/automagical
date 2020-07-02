
pub struct Physics {
    pub velocity: (f32, Orientation),
    pub width: f32,
    pub height: f32
}

impl Physics {
    pub fn new(width: f32, height: f32) -> Physics {
        Physics {
            velocity: (1.0, Orientation::Up),
            width, height
        }
    }

    pub fn generate_movement(&self, time_delta: f32) -> (f32, f32, f32) {
        let (move_x, move_y) = self.velocity.1.positive_axes();
        let x_factor = if move_x {
            1.
        } else {
            0.
        };
        let y_factor = if move_y {
            1.
        } else {
            0.
        };
        (x_factor * time_delta * self.velocity.0, y_factor * time_delta * self.velocity.0, 0.)
    }
}

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

