use bevy::prelude::*;

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
  Idle,
  Going,
  Paused,
  Over,
}
