use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Initialising,
    Countdown,
    Playing,
    GameOver,
    Won,
}
