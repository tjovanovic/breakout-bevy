use bevy::{color::palettes::css, prelude::*};

use crate::{
    ball::Ball,
    bricks::Brick,
    config::{BALL_RADIUS, LIVES_INIT},
    constants::BOTTOM_WALL_EDGE,
    game_sets::GameSet,
};

const COUNTDOWN_TIMER_SECONDS: u8 = 3;

pub struct GameFlowPlugin;
impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(Lives(LIVES_INIT))
            .insert_resource(CountdownTimer(Timer::from_seconds(
                COUNTDOWN_TIMER_SECONDS as f32,
                TimerMode::Once,
            )))
            .add_systems(
                OnEnter(GameState::Initialising),
                (start_countdown, reset_lives),
            )
            .add_systems(
                OnEnter(GameState::Countdown),
                (spawn_countdown_text, reset_countdown_timer),
            )
            .add_systems(Update, countdown.run_if(in_state(GameState::Countdown)))
            .add_systems(OnExit(GameState::Countdown), despawn_countdown_text)
            .add_systems(
                Update,
                (
                    check_win_condition.in_set(GameSet::Outcome),
                    handle_life_loss.in_set(GameSet::Outcome),
                ),
            )
            .add_systems(OnEnter(GameState::GameOver), spawn_end_message)
            .add_systems(OnEnter(GameState::Won), spawn_end_message)
            .add_systems(
                Update,
                end_game_input
                    .run_if(in_state(GameState::GameOver).or_else(in_state(GameState::Won))),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_end_message)
            .add_systems(OnExit(GameState::Won), despawn_end_message);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Initialising,
    Countdown,
    Playing,
    GameOver,
    Won,
}

#[derive(Resource)]
struct Lives(u8);

#[derive(Resource)]
struct CountdownTimer(Timer);

#[derive(Component)]
struct CountdownText;

#[derive(Component)]
struct EndText;

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

fn reset_countdown_timer(mut timer: ResMut<CountdownTimer>) {
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

fn handle_life_loss(
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
