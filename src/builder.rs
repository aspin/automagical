use bevy::prelude::*;

pub fn animate(
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>
) {
    for (mut timer, mut sprite) in &mut query.iter() {
        if timer.finished {
            let offset = 7;
            let durations = vec![0.5, 0.1, 0.06, 0.1, 0.1, 0.1];
            let sprite_index = (sprite.index - offset + 1) % 6;
            sprite.index = sprite_index + offset;

            timer.reset();
            timer.duration = durations[sprite_index as usize];
        }
    }
}