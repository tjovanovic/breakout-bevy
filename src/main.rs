use bevy::{color::palettes::css, prelude::*, window::WindowResolution};

const WINDOW_WIDTH: f32 = 1920.;
const WINDOW_HEIGHT: f32 = 1080.;

const PADDLE_WIDTH: f32 = 100.;
const PADDLE_HEIGHT: f32 = 15.;
const PADDLE_SPEED: f32 = 1000.;
const PADDLE_MARGIN_Y: f32 = 50.;
const PADDLE_MARGIN_X: f32 = 30.;
const PADDLE_MAX_X: f32 = WINDOW_WIDTH / 2. - PADDLE_WIDTH / 2. - PADDLE_MARGIN_X;

const BALL_RADIUS: f32 = 20.;
const BALL_SPEED: f32 = 1500.;
const BALL_INITIAL_ANGLE: f32 = -90.;

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
        .add_systems(
            Update,
            (
                paddle_input,
                apply_velocity.after(paddle_input),
                handle_paddle_edges.after(apply_velocity),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
        Velocity(Vec2::ZERO),
    ));

    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(css::SKY_BLUE))),
        Transform::default(),
        Velocity(Vec2::from_angle(BALL_INITIAL_ANGLE.to_radians()) * BALL_SPEED),
    ));
}

fn paddle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Paddle>>,
) {
    let is_left_pressed = keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let is_right_pressed = keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let mut new_paddle_velocity = Vec2::ZERO;

    if is_left_pressed {
        new_paddle_velocity.x -= PADDLE_SPEED;
    }

    if is_right_pressed {
        new_paddle_velocity.x += PADDLE_SPEED;
    }

    let Ok(mut paddle_velocity) = query.single_mut() else {
        return;
    };

    paddle_velocity.0 = new_paddle_velocity;
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.) * time.delta_secs();
    }
}

fn handle_paddle_edges(mut query: Query<&mut Transform, With<Paddle>>) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };
    transform.translation.x = transform.translation.x.clamp(-PADDLE_MAX_X, PADDLE_MAX_X);
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity(Vec2);
