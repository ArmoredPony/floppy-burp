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
        FixedPostUpdate,
        resume_game.run_if(not(in_state(GameState::Going))),
      )
      .add_systems(
        FixedPostUpdate,
        pause_game.run_if(in_state(GameState::Going)),
      );
  }
}

fn resume_game(
  mut next_state: ResMut<NextState<GameState>>,
  keys: Res<ButtonInput<KeyCode>>,
) {
  if keys.pressed(KeyCode::Space) {
    next_state.set(GameState::Going)
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
