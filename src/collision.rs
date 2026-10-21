use crate::{
    ball::{BALL_RADIUS, BALL_SPEED, Ball},
    bricks::{BRICK_HEIGHT, BRICK_WIDTH, Brick},
    game_sets::GameSet,
    movement::Velocity,
    paddle::{PADDLE_HEIGHT, PADDLE_WIDTH, Paddle},
};
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

const MIN_BOUNCE_ANGLE_FROM_HORIZONTAL: f32 = 30.;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_paddle_ball_collision.in_set(GameSet::Collision),
                handle_brick_ball_collision.in_set(GameSet::Collision),
            ),
        );
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
        let out_angle = offset.remap(
            -1.,
            1.,
            180. - MIN_BOUNCE_ANGLE_FROM_HORIZONTAL,
            MIN_BOUNCE_ANGLE_FROM_HORIZONTAL,
        );
        ball_velocity.0 = Vec2::from_angle(out_angle.to_radians()) * BALL_SPEED;
    }
}
