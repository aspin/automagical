use bevy::prelude::*;

pub fn window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: "Automagical".to_string(),
        height: 2160.,
        width: 3840.,
        ..Default::default()
    }
}
