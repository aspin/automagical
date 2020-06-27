use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use crate::entities::CoreBuilder;

#[derive(SystemDesc)]
pub struct BuilderSystem;

impl<'s> System<'s> for BuilderSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, CoreBuilder>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transforms, builders, input): Self::SystemData) {
        for (builder, transform) in (&builders, &mut transforms).join() {
            if let Some(mv_amount) = input.axis_value("vertical") {
                let builder_y = transform.translation().y;
                transform.set_translation_y(builder_y + mv_amount * builder.speed);
            }
            if let Some(mv_amount) = input.axis_value("horizontal") {
                let builder_yx = transform.translation().x;
                transform.set_translation_x(builder_yx + mv_amount * builder.speed);
            }
            println!("coordinates: x {} y {}", transform.translation().x, transform.translation().y);
        }
    }
}
