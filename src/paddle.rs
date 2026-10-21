use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game_flow::GameState;
use crate::game_sets::GameSet;
use crate::movement::Velocity;

use bevy::color::palettes::css;
use bevy::prelude::*;

pub const PADDLE_WIDTH: f32 = 150.;
pub const PADDLE_HEIGHT: f32 = 15.;
const PADDLE_SPEED: f32 = 700.;
const PADDLE_MARGIN_Y: f32 = 50.;
const PADDLE_MARGIN_X: f32 = 30.;
const PADDLE_MAX_X: f32 = WINDOW_WIDTH / 2. - PADDLE_WIDTH / 2. - PADDLE_MARGIN_X;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Countdown),
            (despawn_paddle, spawn_paddle).chain(),
        )
        .add_systems(
            Update,
            (
                handle_paddle_input.in_set(GameSet::Input),
                handle_paddle_edges.in_set(GameSet::Collision),
            ),
        );
    }
}

#[derive(Component)]
pub struct Paddle;

fn spawn_paddle(mut commands: Commands) {
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
}

fn despawn_paddle(mut commands: Commands, query: Query<Entity, With<Paddle>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

fn handle_paddle_input(
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

fn handle_paddle_edges(mut query: Query<&mut Transform, With<Paddle>>) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };
    transform.translation.x = transform.translation.x.clamp(-PADDLE_MAX_X, PADDLE_MAX_X);
}
