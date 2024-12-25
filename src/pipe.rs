use bevy::{prelude::*, sprite::Anchor};

use crate::{collision::Shape, GAME_SPEED};

pub const HITBOX_SIZE: Vec2 = Vec2 { x: 17.0, y: 142.0 };
pub const VERTICAL_GAP: f32 = 50.0;

pub struct PipePlugin;

impl Plugin for PipePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(PipeSpawnTimer::from_period(1.0))
      .add_systems(Update, tick_spawn_timer)
      .add_systems(Update, spawn_pipes)
      .add_systems(FixedUpdate, update_pipes);
  }
}

#[derive(Component)]
pub struct Pipe;

#[derive(Resource, Deref, DerefMut)]
pub struct PipeSpawnTimer(pub Timer);

impl PipeSpawnTimer {
  pub fn from_period(period: f32) -> Self {
    Self(Timer::from_seconds(period, TimerMode::Repeating))
  }
}

fn tick_spawn_timer(time: Res<Time>, mut spawn_timer: ResMut<PipeSpawnTimer>) {
  spawn_timer.tick(time.delta());
}

fn spawn_pipes(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  spawn_timer: Res<PipeSpawnTimer>,
) {
  if spawn_timer.finished() {
    commands.spawn((
      Pipe,
      Sprite {
        image: asset_server.load("pipe.png"),
        anchor: Anchor::TopCenter,
        ..default()
      },
      Transform::from_xyz(0.0, -VERTICAL_GAP / 2.0, 0.0),
      Shape::Rectangle(Rectangle::from_size(HITBOX_SIZE)),
      Anchor::TopCenter,
    ));
  }
}

fn update_pipes(time: Res<Time>, mut query: Query<&mut Transform, With<Pipe>>) {
  for mut transform in &mut query {
    transform.translation.x -= GAME_SPEED * time.delta_secs();
  }
}
