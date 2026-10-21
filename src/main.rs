use bevy::{prelude::*, window::WindowResolution};

use bricks::BricksPlugin;
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game_sets::GameSetPlugin;
use movement::MovementPlugin;
use paddle::PaddlePlugin;

use crate::{ball::BallPlugin, collision::CollisionPlugin, game_flow::GameFlowPlugin};

mod ball;
mod bricks;
mod collision;
mod config;
mod constants;
mod game_flow;
mod game_sets;
mod movement;
mod paddle;

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
        .add_systems(Startup, setup_camera)
        .add_plugins(GameSetPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(BricksPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(GameFlowPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
