use std::f32::consts::PI;

use bevy::{
    color::palettes::css,
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
    window::WindowResolution,
};

use rand::RngExt;

const WINDOW_WIDTH: f32 = 1920.;
const WINDOW_HEIGHT: f32 = 1080.;

const PADDLE_WIDTH: f32 = 150.;
const PADDLE_HEIGHT: f32 = 15.;
const PADDLE_SPEED: f32 = 300.;
const PADDLE_MARGIN_Y: f32 = 50.;
const PADDLE_MARGIN_X: f32 = 30.;
const PADDLE_MAX_X: f32 = WINDOW_WIDTH / 2. - PADDLE_WIDTH / 2. - PADDLE_MARGIN_X;

const BALL_RADIUS: f32 = 15.;
const BALL_SPEED: f32 = 300.;
const BALL_INITIAL_ANGLE_RANGE: (f32, f32) = (-120., -60.);

const LEFT_WALL_EDGE: f32 = -WINDOW_WIDTH / 2.;
const RIGHT_WALL_EDGE: f32 = WINDOW_WIDTH / 2.;
const BOTTOM_WALL_EDGE: f32 = -WINDOW_HEIGHT / 2.;
const TOP_WALL_EDGE: f32 = WINDOW_HEIGHT / 2.;

const MAX_BOUNCE_ANGLE: f32 = 30.;

const BRICK_ROWS: u8 = 4;
const BRICK_COLS: u8 = 10;
const BRICK_Y_GAP: f32 = 20.;
const BRICK_X_GAP: f32 = 25.;

const BRICK_WIDTH: f32 =
    (WINDOW_WIDTH - (BRICK_COLS as f32 + 1.) * BRICK_X_GAP) / (BRICK_COLS as f32);
const BRICK_HEIGHT: f32 = 50.;

const BALL_ANGLE_CORRECTION_MARGIN: f32 = 1. / 18. * PI; // 10 degrees

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Countdown,
    Playing,
    GameOver,
    Won,
}

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
        .init_state::<GameState>()
        .add_systems(Startup, (setup_camera, setup_bricks))
        .add_systems(
            OnEnter(GameState::Countdown),
            (despawn_ball_and_paddle, spawn_ball_and_paddle).chain(),
        )
        .add_systems(
            Update,
            (
                paddle_input,
                apply_velocity.after(paddle_input),
                handle_paddle_edges.after(apply_velocity),
                handle_ball_edges.after(apply_velocity),
                handle_paddle_ball_collision.after(apply_velocity),
                handle_brick_ball_collision.after(apply_velocity),
                correct_ball_angle
                    .after(handle_ball_edges)
                    .after(handle_paddle_ball_collision)
                    .after(handle_brick_ball_collision),
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn despawn_ball_and_paddle(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Ball>, With<Paddle>)>>,
) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

fn spawn_ball_and_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    let mut rng = rand::rng();
    let ball_initial_angle =
        rng.random_range(BALL_INITIAL_ANGLE_RANGE.0..=BALL_INITIAL_ANGLE_RANGE.1);

    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(css::SKY_BLUE))),
        Transform::default(),
        Velocity(Vec2::from_angle(ball_initial_angle.to_radians()) * BALL_SPEED),
    ));
}

fn setup_bricks(mut commands: Commands) {
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let row = row as f32;
            let col = col as f32;
            let brick_x = -WINDOW_WIDTH / 2.
                + (col + 1.) * BRICK_X_GAP
                + col * BRICK_WIDTH
                + BRICK_WIDTH / 2.;
            let brick_y = WINDOW_HEIGHT / 2.
                - (row + 1.) * BRICK_Y_GAP
                - row * BRICK_HEIGHT
                - BRICK_HEIGHT / 2.;

            commands.spawn((
                Brick,
                Sprite {
                    custom_size: Some(Vec2::new(BRICK_WIDTH, BRICK_HEIGHT)),
                    color: css::INDIAN_RED.into(),
                    ..default()
                },
                Transform {
                    translation: vec3(brick_x, brick_y, 0.),
                    ..default()
                },
            ));
        }
    }
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

fn handle_ball_edges(mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>) {
    let Ok((mut transform, mut velocity)) = query.single_mut() else {
        return;
    };

    let left_ball_edge = transform.translation.x - BALL_RADIUS;
    if left_ball_edge <= LEFT_WALL_EDGE {
        velocity.0.x = -velocity.0.x;
        transform.translation.x = LEFT_WALL_EDGE + BALL_RADIUS;
    }

    let right_ball_edge = transform.translation.x + BALL_RADIUS;
    if right_ball_edge >= RIGHT_WALL_EDGE {
        velocity.0.x = -velocity.0.x;
        transform.translation.x = RIGHT_WALL_EDGE - BALL_RADIUS;
    }

    let bottom_ball_edge = transform.translation.y - BALL_RADIUS;
    if bottom_ball_edge <= BOTTOM_WALL_EDGE {
        velocity.0.y = -velocity.0.y;
        transform.translation.y = BOTTOM_WALL_EDGE + BALL_RADIUS;
    }

    let top_ball_edge = transform.translation.y + BALL_RADIUS;
    if top_ball_edge >= TOP_WALL_EDGE {
        velocity.0.y = -velocity.0.y;
        transform.translation.y = TOP_WALL_EDGE - BALL_RADIUS;
    }
}

fn handle_brick_ball_collision(
    mut commands: Commands,
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    brick_query: Query<(Entity, &Transform), With<Brick>>,
) {
    let Ok((ball_transform, mut ball_velocity)) = ball_query.single_mut() else {
        return;
    };

    let ball = BoundingCircle::new(ball_transform.translation.xy(), BALL_RADIUS);
    for (brick_entity, brick_transform) in brick_query {
        let brick = Aabb2d::new(
            brick_transform.translation.xy(),
            vec2(BRICK_WIDTH / 2., BRICK_HEIGHT / 2.),
        );
        if ball.intersects(&brick) {
            commands.entity(brick_entity).despawn();

            let closest = brick.closest_point(ball.center);
            let Vec2 { x: hit_x, y: hit_y } = ball.center - closest;
            if hit_x.abs() > hit_y.abs() {
                ball_velocity.0.x = if hit_x > 0. {
                    ball_velocity.0.x.abs()
                } else {
                    -ball_velocity.0.x.abs()
                };
            } else {
                ball_velocity.0.y = if hit_y > 0. {
                    ball_velocity.0.y.abs()
                } else {
                    -ball_velocity.0.y.abs()
                };
            }
        }
    }
}

fn handle_paddle_ball_collision(
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
) {
    let Ok((ball_transform, mut ball_velocity)) = ball_query.single_mut() else {
        return;
    };

    let Ok(paddle_transform) = paddle_query.single() else {
        return;
    };

    if ball_velocity.0.y >= 0. {
        // as the ball can remain inside the paddle for a few frames after flip
        // we only change the velocity if the ball is going down
        // to ensure we only do it once
        return;
    }

    let paddle = Aabb2d::new(
        paddle_transform.translation.xy(),
        vec2(PADDLE_WIDTH / 2., PADDLE_HEIGHT / 2.),
    );

    let ball = BoundingCircle::new(ball_transform.translation.xy(), BALL_RADIUS);

    if ball.intersects(&paddle) {
        let mut offset =
            (ball_transform.translation.x - paddle_transform.translation.x) / (PADDLE_WIDTH / 2.);

        offset = offset.clamp(-1., 1.);
        let out_angle = offset.remap(-1., 1., 180. - MAX_BOUNCE_ANGLE, MAX_BOUNCE_ANGLE);
        ball_velocity.0 = Vec2::from_angle(out_angle.to_radians()) * BALL_SPEED;
    }
}

fn correct_ball_angle(mut query: Query<&mut Velocity, With<Ball>>) {
    let Ok(mut velocity) = query.single_mut() else {
        return;
    };

    let current_angle = velocity.0.to_angle();
    let Vec2 { x: curr_x, .. } = velocity.0;

    let deg_90 = PI / 2.;
    if (deg_90 - current_angle).abs() <= BALL_ANGLE_CORRECTION_MARGIN {
        let new_angle = if curr_x >= 0. {
            deg_90 - BALL_ANGLE_CORRECTION_MARGIN
        } else {
            deg_90 + BALL_ANGLE_CORRECTION_MARGIN
        };
        velocity.0 = Vec2::from_angle(new_angle) * BALL_SPEED;
    }

    if (-deg_90 - current_angle).abs() <= BALL_ANGLE_CORRECTION_MARGIN {
        let new_angle = if curr_x >= 0. {
            -deg_90 + BALL_ANGLE_CORRECTION_MARGIN
        } else {
            -deg_90 - BALL_ANGLE_CORRECTION_MARGIN
        };
        velocity.0 = Vec2::from_angle(new_angle) * BALL_SPEED;
    }
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Brick;
