use bevy::prelude::*;
use rand::Rng;

use crate::{collision::Shape, layer::Layer, GAME_SPEED, RESOLUTION};

pub const HITBOX_SIZE: Vec2 = Vec2 { x: 17.0, y: 142.0 };
pub const VERTICAL_GAP: f32 = 30.0;
pub const SPAWN_POINT_MID_DISTANCE: f32 = 40.0;
pub const RESPAWN_COOLDOWN_SEC: f32 = 2.0;

pub struct PipePlugin;

impl Plugin for PipePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(PipeSpawnTimer::from_period(RESPAWN_COOLDOWN_SEC))
      .add_systems(Update, tick_spawn_timer)
      .add_systems(Update, spawn_pipes)
      .add_systems(Update, update_pipes)
      .add_systems(Update, despawn_pipes);
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
    let spawn_point_y = rand::thread_rng()
      .gen_range((-SPAWN_POINT_MID_DISTANCE)..(SPAWN_POINT_MID_DISTANCE));
    commands.spawn((
      Pipe,
      Sprite {
        image: asset_server.load("pipe.png"),
        flip_y: true,
        ..default()
      },
      Transform::from_xyz(
        RESOLUTION.x / 2.0 + HITBOX_SIZE.x,
        (VERTICAL_GAP + HITBOX_SIZE.y) / 2.0 + spawn_point_y,
        Layer::Pipe.into(),
      ),
      Shape::Rectangle(Rectangle::from_size(HITBOX_SIZE)),
    ));
    commands.spawn((
      Pipe,
      Sprite {
        image: asset_server.load("pipe.png"),
        ..default()
      },
      Transform::from_xyz(
        RESOLUTION.x / 2.0 + HITBOX_SIZE.x,
        -(VERTICAL_GAP + HITBOX_SIZE.y) / 2.0 + spawn_point_y,
        Layer::Pipe.into(),
      ),
      Shape::Rectangle(Rectangle::from_size(HITBOX_SIZE)),
    ));
  }
}

fn update_pipes(time: Res<Time>, mut query: Query<&mut Transform, With<Pipe>>) {
  for mut transform in &mut query {
    transform.translation.x -= GAME_SPEED * time.delta_secs();
  }
}

fn despawn_pipes(
  mut commands: Commands,
  query: Query<(Entity, &Transform), With<Pipe>>,
) {
  for (pipe, transform) in &query {
    if transform.translation.x <= -RESOLUTION.x / 2.0 - HITBOX_SIZE.x {
      dbg!(transform.translation.x);
      commands.entity(pipe).despawn();
    }
  }
}
