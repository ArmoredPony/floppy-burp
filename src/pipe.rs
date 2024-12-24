use bevy::prelude::*;

pub struct PipePlugin;

impl Plugin for PipePlugin {
  fn build(&self, app: &mut App) {
    todo!()
  }
}

#[derive(Component)]
pub struct Pipe {
  is_bottom: bool,
}

#[derive(Resource, Deref)]
pub struct PipeSpawnTimer(pub Timer);

fn spawn_pipes(mut commands: Commands) {}
