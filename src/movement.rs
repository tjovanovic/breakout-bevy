use crate::game_sets::GameSet;
use bevy::prelude::*;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity.in_set(GameSet::Movement));
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.) * time.delta_secs();
    }
}
