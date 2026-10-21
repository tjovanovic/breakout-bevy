use bevy::color::palettes::css;
use bevy::prelude::*;

use crate::config::{BRICK_COLS, BRICK_ROWS};
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game_flow::GameState;

const BRICK_Y_GAP: f32 = 20.;
const BRICK_X_GAP: f32 = 25.;

pub const BRICK_WIDTH: f32 =
    (WINDOW_WIDTH - (BRICK_COLS as f32 + 1.) * BRICK_X_GAP) / (BRICK_COLS as f32);
pub const BRICK_HEIGHT: f32 = 50.;

pub struct BricksPlugin;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Initialising),
            (despawn_bricks, spawn_bricks).chain(),
        );
    }
}

#[derive(Component)]
pub struct Brick;

fn spawn_bricks(mut commands: Commands) {
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

fn despawn_bricks(mut commands: Commands, query: Query<Entity, With<Brick>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}
