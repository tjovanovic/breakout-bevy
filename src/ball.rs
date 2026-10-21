use crate::config::{BALL_RADIUS, BALL_SPEED};
use crate::constants::{LEFT_WALL_EDGE, RIGHT_WALL_EDGE, TOP_WALL_EDGE};
use crate::game_flow::GameState;
use crate::game_sets::GameSet;
use crate::movement::Velocity;
use bevy::color::palettes::css;
use bevy::prelude::*;
use rand::RngExt;
use std::f32::consts::PI;

const BALL_INITIAL_ANGLE_RANGE: (f32, f32) = (-120., -60.);
const BALL_ANGLE_CORRECTION_MARGIN: f32 = 1. / 18. * PI; // 10 degrees
const BALL_INITIAL_Y_OFFSET: f32 = 200.;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Countdown),
            (despawn_ball, spawn_ball).chain(),
        )
        .add_systems(
            Update,
            (
                handle_ball_wall_collision.in_set(GameSet::Collision),
                correct_ball_angle.in_set(GameSet::Correction),
            ),
        );
    }
}

#[derive(Component)]
pub struct Ball;

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

fn despawn_ball(mut commands: Commands, query: Query<Entity, With<Ball>>) {
    for entity in query {
        commands.entity(entity).despawn();
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
