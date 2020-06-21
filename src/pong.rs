use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    core::timing::Time,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub const BALL_RADIUS: f32 = 4.0;
pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const RESPAWN_BALL_INTERVAL: f32 = 1.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }

    fn half_width(&self) -> f32 {
        return self.width / 2.0;
    }

    fn half_height(&self) -> f32 {
        return self.height / 2.0;
    }

    pub fn ball_collides(&self, paddle_transform: &Transform, ball_transform: &Transform) -> bool {
        let anchor_x = paddle_transform.translation().x;
        let anchor_y = paddle_transform.translation().y;

        let left_x = anchor_x - self.half_width();
        let right_x = anchor_x + self.half_width();
        let bottom_y = anchor_y - self.half_height();
        let top_y = anchor_y + self.half_height();

        let ball_anchor_x = ball_transform.translation().x;
        let ball_anchor_y = ball_transform.translation().y;

        let matches_y = ball_anchor_y >= bottom_y && ball_anchor_y <= top_y;
        let matches_x = ball_anchor_x >= left_x && ball_anchor_x <= right_x;
        return matches_y && matches_x;
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
    pub radius: f32,
    pub velocity: (f32, f32),
    pub reset_time_countdown: Option<f32>,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            radius: BALL_RADIUS,
            velocity: (BALL_VELOCITY_X, BALL_VELOCITY_Y),
            reset_time_countdown: None,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Score {
    pub left: u32,
    pub right: u32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

#[derive(Default)]
pub struct MagicalPong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for MagicalPong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ball_spawn_timer.replace(1.0);

        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialize_camera(world);
        initialize_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialize_scoreboard(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            timer -= data.world.fetch::<Time>().delta_seconds();
            if timer <= 0.0 {
                initialize_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .with(sprite_render.clone())
        .build();
}

fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(Ball::new())
        .with(transform)
        .with(sprite_render.clone())
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        50.,
        -50.,
        1.,
        200.,
        50.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.insert(ScoreText { p1_score, p2_score })
}
