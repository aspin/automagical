const DEFAULT_BUILDER_SPEED: f32 = 100.0;

pub struct Builder {
    pub speed: f32
}

impl Builder {
    fn new() -> Builder {
        Builder {
            speed: DEFAULT_BUILDER_SPEED
        }
    }
}
