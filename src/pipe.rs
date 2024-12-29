use bevy::prelude::*;
use rand::Rng;

use crate::{
  checkpoint::Checkpoint,
  collision::Shape,
  ground::Ground,
  layer::Layer,
  state::GameState,
  GAME_SPEED,
  RESOLUTION,
};

pub struct PipePlugin;

impl Plugin for PipePlugin {
  fn build(&self, app: &mut App) {
    let pipe_reset_system_set = (
      reset_spawn_timer, //
      despawn_all_pipes,
      spawn_pipes_randomly,
    );
    app
      .insert_resource(PipeSpawnTimer::from_period(Pipe::RESPAWN_COOLDOWN_SEC))
      .add_systems(OnExit(GameState::Idle), pipe_reset_system_set)
      .add_systems(OnExit(GameState::GameOver), pipe_reset_system_set)
      .add_systems(
        Update,
        (
          tick_spawn_timer,
          spawn_pipes_periodically,
          update_pipes,
          despawn_out_of_bounds_pipes,
        )
          .run_if(in_state(GameState::Going)),
      );
  }
}

#[derive(Component)]
pub struct Pipe;

impl Pipe {
  pub const HITBOX_SIZE: Vec2 = Vec2 { x: 17.0, y: 142.0 };
  pub const VERTICAL_GAP: f32 = 35.0;
  pub const SPAWN_POINT_MID_DISTANCE: f32 = 50.0;
  pub const RESPAWN_COOLDOWN_SEC: f32 = 2.0;
}

#[derive(Resource, Deref, DerefMut)]
pub struct PipeSpawnTimer(pub Timer);

impl PipeSpawnTimer {
  pub fn from_period(period: f32) -> Self {
    Self(Timer::from_seconds(period, TimerMode::Repeating))
  }
}

fn reset_spawn_timer(mut spawn_timer: ResMut<PipeSpawnTimer>) {
  spawn_timer.reset();
}

fn tick_spawn_timer(time: Res<Time>, mut spawn_timer: ResMut<PipeSpawnTimer>) {
  spawn_timer.tick(time.delta());
}

fn spawn_pipes_randomly(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  let image = asset_server.load::<Image>("pipe.png");
  let shape = Shape::Rectangle(Rectangle::from_size(Pipe::HITBOX_SIZE));
  let spawn_point = Vec2 {
    x: RESOLUTION.x / 2.0 + Pipe::HITBOX_SIZE.x,
    y: rand::thread_rng().gen_range(
      (-Pipe::SPAWN_POINT_MID_DISTANCE + Ground::LEVEL)
        ..Pipe::SPAWN_POINT_MID_DISTANCE,
    ),
  };
  commands.spawn((
    Pipe,
    Sprite {
      image: image.clone(),
      flip_y: true,
      ..default()
    },
    Transform::from_xyz(
      spawn_point.x,
      spawn_point.y + (Pipe::VERTICAL_GAP + Pipe::HITBOX_SIZE.y) / 2.0,
      Layer::Pipe.into(),
    ),
    shape,
  ));
  commands.spawn((
    Pipe,
    Sprite::from_image(image),
    Transform::from_xyz(
      spawn_point.x,
      spawn_point.y - (Pipe::VERTICAL_GAP + Pipe::HITBOX_SIZE.y) / 2.0,
      Layer::Pipe.into(),
    ),
    shape,
    Checkpoint,
  ));
}

fn spawn_pipes_periodically(
  commands: Commands,
  asset_server: Res<AssetServer>,
  spawn_timer: Res<PipeSpawnTimer>,
) {
  if spawn_timer.finished() {
    spawn_pipes_randomly(commands, asset_server);
  }
}

fn update_pipes(time: Res<Time>, mut query: Query<&mut Transform, With<Pipe>>) {
  for mut transform in &mut query {
    transform.translation.x -= GAME_SPEED * time.delta_secs();
  }
}

fn despawn_out_of_bounds_pipes(
  mut commands: Commands,
  query: Query<(Entity, &Transform), With<Pipe>>,
) {
  for (pipe, transform) in &query {
    if transform.translation.x <= -RESOLUTION.x / 2.0 - Pipe::HITBOX_SIZE.x {
      commands.entity(pipe).despawn();
    }
  }
}

fn despawn_all_pipes(mut commands: Commands, query: Query<Entity, With<Pipe>>) {
  for pipe in &query {
    commands.entity(pipe).despawn();
  }
}
