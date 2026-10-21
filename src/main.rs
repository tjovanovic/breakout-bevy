use std::f32::consts::PI;

use bevy::{
    color::palettes::css,
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
    window::WindowResolution,
};
use rand::RngExt;

use bricks::{BRICK_HEIGHT, BRICK_WIDTH, Brick, BricksPlugin};
use constants::{
    BOTTOM_WALL_EDGE, LEFT_WALL_EDGE, RIGHT_WALL_EDGE, TOP_WALL_EDGE, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use game_flow::GameState;
use game_sets::{GameSet, GameSetPlugin};
use movement::{MovementPlugin, Velocity};
use paddle::{PADDLE_HEIGHT, PADDLE_WIDTH, Paddle, PaddlePlugin};

mod bricks;
mod constants;
mod game_flow;
mod game_sets;
mod movement;
mod paddle;

const LIVES_INIT: u8 = 3;

const BALL_RADIUS: f32 = 15.;
const BALL_SPEED: f32 = 700.;
const BALL_INITIAL_ANGLE_RANGE: (f32, f32) = (-120., -60.);
const BALL_ANGLE_CORRECTION_MARGIN: f32 = 1. / 18. * PI; // 10 degrees
const BALL_INITIAL_Y_OFFSET: f32 = 200.;

const MAX_BOUNCE_ANGLE: f32 = 30.;

const COUNTDOWN_TIMER_SECONDS: u8 = 3;

#[derive(Resource)]
struct Lives(u8);

#[derive(Resource)]
struct CountdownTimer(Timer);

#[derive(Component)]
struct CountdownText;

#[derive(Component)]
struct EndText;

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
        .add_plugins(GameSetPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(BricksPlugin)
        .add_plugins(PaddlePlugin)
        .init_state::<GameState>()
        .insert_resource(Lives(LIVES_INIT))
        .insert_resource(CountdownTimer(Timer::from_seconds(
            COUNTDOWN_TIMER_SECONDS as f32,
            TimerMode::Once,
        )))
        .add_systems(Startup, setup_camera)
        .add_systems(
            OnEnter(GameState::Initialising),
            (start_countdown, reset_lives),
        )
        .add_systems(
            OnEnter(GameState::Countdown),
            (
                (despawn_ball, spawn_ball).chain(),
                spawn_countdown_text,
                reset_timer,
            ),
        )
        .add_systems(Update, countdown.run_if(in_state(GameState::Countdown)))
        .add_systems(OnExit(GameState::Countdown), despawn_countdown_text)
        .add_systems(
            Update,
            (
                check_win_condition.in_set(GameSet::Outcome),
                handle_ball_wall_collision.in_set(GameSet::Collision),
                handle_paddle_ball_collision.in_set(GameSet::Collision),
                handle_brick_ball_collision.in_set(GameSet::Collision),
                handle_ball_death.in_set(GameSet::Outcome),
                correct_ball_angle.in_set(GameSet::Correction),
            ),
        )
        .add_systems(OnEnter(GameState::GameOver), spawn_end_message)
        .add_systems(OnEnter(GameState::Won), spawn_end_message)
        .add_systems(
            Update,
            end_game_input.run_if(in_state(GameState::GameOver).or_else(in_state(GameState::Won))),
        )
        .add_systems(OnExit(GameState::GameOver), despawn_end_message)
        .add_systems(OnExit(GameState::Won), despawn_end_message)
        .run();
}

fn spawn_end_message(mut commands: Commands, state: Res<State<GameState>>) {
    let message = match state.get() {
        GameState::GameOver => "Game Over",
        GameState::Won => "Victory!",
        _ => return,
    };

    commands.spawn((
        Text2d::new(message),
        TextFont {
            font_size: FontSize::Px(120.0),
            ..Default::default()
        },
        TextColor(css::SKY_BLUE.into()),
        EndText,
    ));
}

fn despawn_end_message(mut commands: Commands, query: Query<Entity, With<EndText>>) {
    let Ok(entity) = query.single() else {
        return;
    };
    commands.entity(entity).despawn();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn countdown(
    time: Res<Time>,
    mut timer: ResMut<CountdownTimer>,
    mut query: Query<&mut Text2d, With<CountdownText>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = timer.0.remaining_secs().ceil().to_string();
    }

    timer.0.tick(time.delta());
    if timer.0.is_finished() {
        next_game_state.set(GameState::Playing);
    }
}

fn reset_timer(mut timer: ResMut<CountdownTimer>) {
    timer.0.reset();
}

fn spawn_countdown_text(mut commands: Commands) {
    commands.spawn((
        Text2d::new(COUNTDOWN_TIMER_SECONDS.to_string()),
        TextFont {
            font_size: FontSize::Px(120.0),
            ..Default::default()
        },
        TextColor(css::SKY_BLUE.into()),
        CountdownText,
    ));
}

fn despawn_countdown_text(mut commands: Commands, query: Query<Entity, With<CountdownText>>) {
    let Ok(entity) = query.single() else {
        return;
    };
    commands.entity(entity).despawn();
}

fn despawn_ball(mut commands: Commands, query: Query<Entity, With<Ball>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let ball_initial_angle =
        rng.random_range(BALL_INITIAL_ANGLE_RANGE.0..=BALL_INITIAL_ANGLE_RANGE.1);

    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(css::SKY_BLUE))),
        Transform {
            translation: vec3(0., BALL_INITIAL_Y_OFFSET, 0.),
            ..default()
        },
        Velocity(Vec2::from_angle(ball_initial_angle.to_radians()) * BALL_SPEED),
    ));
}

fn start_countdown(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Countdown);
}

fn reset_lives(mut lives: ResMut<Lives>) {
    lives.0 = LIVES_INIT;
}

fn end_game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_game_state.set(GameState::Initialising);
    }
}

fn handle_ball_wall_collision(mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>) {
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

    let top_ball_edge = transform.translation.y + BALL_RADIUS;
    if top_ball_edge >= TOP_WALL_EDGE {
        velocity.0.y = -velocity.0.y;
        transform.translation.y = TOP_WALL_EDGE - BALL_RADIUS;
    }
}

fn handle_ball_death(
    query: Query<&Transform, With<Ball>>,
    mut lives: ResMut<Lives>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let Ok(transform) = query.single() else {
        return;
    };

    let bottom_ball_edge = transform.translation.y - BALL_RADIUS;
    if bottom_ball_edge <= BOTTOM_WALL_EDGE {
        lives.0 -= 1;
        if lives.0 == 0 {
            next_game_state.set(GameState::GameOver);
        } else {
            next_game_state.set(GameState::Countdown);
        }
    }
}

fn check_win_condition(
    query: Query<(), With<Brick>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if query.is_empty() {
        next_game_state.set(GameState::Won);
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
struct Ball;
