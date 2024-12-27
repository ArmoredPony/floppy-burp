use bevy::prelude::*;

use crate::{bird::Bird, RESOLUTION};

pub struct CheckpointPlugin;

impl Plugin for CheckpointPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<CheckpointPassed>()
      .add_systems(FixedUpdate, update_checkpoints)
      .add_systems(PostUpdate, debug_checkpoints);
  }
}

#[derive(Component)]
pub struct Checkpoint;

#[derive(Event)]
pub struct CheckpointPassed;

fn update_checkpoints(
  mut commands: Commands,
  mut events: EventWriter<CheckpointPassed>,
  bird_transform: Single<&Transform, With<Bird>>,
  mut checkpoints: Query<(Entity, &Transform), With<Checkpoint>>,
) {
  for (checkpoint, checkpoint_transform) in &mut checkpoints {
    if bird_transform.translation.x > checkpoint_transform.translation.x {
      events.send(CheckpointPassed);
      commands.entity(checkpoint).remove::<Checkpoint>();
    }
  }
}

fn debug_checkpoints(
  mut gizmos: Gizmos,
  mut events: EventReader<CheckpointPassed>,
  checkpoints: Query<&Transform, With<Checkpoint>>,
) {
  for checkpoint_transform in &checkpoints {
    gizmos.line_2d(
      checkpoint_transform.translation.xy().with_y(RESOLUTION.y),
      checkpoint_transform.translation.xy().with_y(-RESOLUTION.y),
      Color::srgb(0.0, 1.0, 0.0),
    );
  }
  for _ in events.read() {
    dbg!("Score incremented!");
  }
}
