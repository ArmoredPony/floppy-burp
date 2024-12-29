use bevy::{prelude::*, sprite::Anchor};

use crate::{layer::Layer, state::GameState, GAME_SPEED, RESOLUTION};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
  fn build(&self, app: &mut App) {
    app //
      .add_systems(Startup, init_background)
      .add_systems(
        Update,
        animate_background
          .run_if(in_state(GameState::Idle).or(in_state(GameState::Going))),
      );
  }
}

#[derive(Component)]
struct Background;

impl Background {
  pub const SCROLL_SPEED: f32 = GAME_SPEED / 3.0;
}

fn init_background(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn((
    Background,
    Sprite {
      image: asset_server.load("background-day.png"),
      anchor: Anchor::CenterLeft,
      ..default()
    },
    Transform::from_xyz(-RESOLUTION.x / 2.0, 0.0, Layer::Background.into())
      .with_scale(Vec3::splat(0.7)),
  ));
}

fn animate_background(
  time: Res<Time>,
  mut transform: Single<&mut Transform, With<Background>>,
) {
  transform.translation.x -= time.delta_secs() * Background::SCROLL_SPEED;
  if transform.translation.x <= RESOLUTION.x / 2.0 - 276.0 {
    transform.translation.x = -RESOLUTION.x / 2.0;
  }
}
