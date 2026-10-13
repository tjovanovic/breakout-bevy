use bevy::{color::palettes::css, prelude::*, window::WindowResolution};

const WINDOW_WIDTH: f32 = 1920.;
const WINDOW_HEIGHT: f32 = 1080.;

const PADDLE_WIDTH: f32 = 100.;
const PADDLE_HEIGHT: f32 = 15.;
const PADDLE_SPEED: f32 = 1000.;
const PADDLE_MARGIN_Y: f32 = 50.;
const PADDLE_MARGIN_X: f32 = 30.;
const PADDLE_MAX_X: f32 = WINDOW_WIDTH / 2. - PADDLE_WIDTH / 2. - PADDLE_MARGIN_X;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_paddle)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Paddle,
        Sprite {
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            color: css::INDIAN_RED.into(),
            ..default()
        },
        Transform {
            translation: vec3(
                0.,
                PADDLE_MARGIN_Y + PADDLE_HEIGHT / 2. - WINDOW_HEIGHT / 2.,
                0.,
            ),
            ..default()
        },
    ));
}

fn move_paddle(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let is_left_pressed = keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let is_right_pressed = keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let total_movement = PADDLE_SPEED * time.delta_secs();
    let mut movement_x = 0.;

    if is_left_pressed {
        movement_x -= total_movement;
    }

    if is_right_pressed {
        movement_x += total_movement;
    }

    let Ok(mut paddle) = query.single_mut() else {
        return;
    };

    if movement_x == 0. {
        return;
    }
    paddle.translation.x += movement_x;
    paddle.translation.x = paddle.translation.x.clamp(-PADDLE_MAX_X, PADDLE_MAX_X);
}

#[derive(Component)]
struct Paddle;
