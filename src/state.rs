use std::ops::Not;

use bevy::prelude::*;

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
  Idle,
  Going,
  Paused,
  GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
  fn build(&self, app: &mut App) {
    app //
      .add_systems(
        FixedUpdate,
        (
          resume_game.run_if(not(in_state(GameState::Going))),
          pause_game.run_if(in_state(GameState::Going)),
        ),
      );
  }
}

#[derive(Default, Debug)]
struct IsPaused(bool);

fn resume_game(
  mut is_paused: Local<IsPaused>,
  mut next_state: ResMut<NextState<GameState>>,
  keys: Res<ButtonInput<KeyCode>>,
) {
  if is_paused.0.not() {
    if keys.pressed(KeyCode::Space).not() {
      is_paused.0 = true;
    }
  } else if keys.pressed(KeyCode::Space) {
    next_state.set(GameState::Going);
    is_paused.0 = false;
  }
}

fn pause_game(
  mut next_state: ResMut<NextState<GameState>>,
  keys: Res<ButtonInput<KeyCode>>,
) {
  if keys.pressed(KeyCode::Escape) {
    next_state.set(GameState::Paused);
  }
}
