use bevy::prelude::*;

use crate::{checkpoint::CheckpointPassed, state::GameState};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<Score>()
      .add_systems(OnExit(GameState::Idle), reset_score)
      .add_systems(OnExit(GameState::GameOver), reset_score)
      .add_systems(FixedUpdate, increment_score);
  }
}

#[derive(Resource, Default)]
pub struct Score(u32);

fn increment_score(
  mut score: ResMut<Score>,
  mut checkpoint_events: EventReader<CheckpointPassed>,
) {
  let score_delta = checkpoint_events.read().count() as u32;
  if score_delta > 0 {
    score.0 = score.0.saturating_add(score_delta);
    debug!("score is {}", score.0);
  }
}

fn reset_score(mut score: ResMut<Score>) {
  score.0 = 0;
}
