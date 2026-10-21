use crate::game_flow::GameState;
use bevy::prelude::*;

pub struct GameSetPlugin;
impl Plugin for GameSetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSet::Input,
                GameSet::Movement,
                GameSet::Collision,
                GameSet::Correction,
                GameSet::Outcome,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet {
    Input,
    Movement,
    Collision,
    Correction,
    Outcome,
}
