use bevy::prelude::*;

use crate::{
  collision::Shape,
  layer::Layer,
  state::GameState,
  GAME_SPEED,
  RESOLUTION,
};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init_ground).add_systems(
      PostUpdate,
      animate_ground
        .run_if(in_state(GameState::Idle).or(in_state(GameState::Going))),
    );
  }
}

#[derive(Component)]
pub struct Ground;

impl Ground {
  pub const LEVEL: f32 = 50.0;
  pub const SEGMENT_WIDTH: f32 = 18.0;
}

fn init_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
  let rectangle = Rectangle::new(RESOLUTION.x * 2.0, Ground::LEVEL);
  commands.spawn((
    Ground,
    Transform::from_xyz(
      rectangle.half_size.x / 2.0,
      (-RESOLUTION.y + Ground::LEVEL) / 2.0,
      Layer::Ground.into(),
    ),
    Sprite {
      image: asset_server.load("base.png"),
      rect: Some(Rect::from_center_half_size(
        rectangle.half_size.with_x(RESOLUTION.x),
        rectangle.half_size.with_x(RESOLUTION.x),
      )),
      ..default()
    },
    Shape::Rectangle(rectangle),
  ));
}

fn animate_ground(
  time: Res<Time>,
  mut transform: Single<&mut Transform, With<Ground>>,
) {
  transform.translation.x -= time.delta_secs() * GAME_SPEED;
  if transform.translation.x <= -RESOLUTION.x / 2.0 {
    transform.translation.x = 0.0 - Ground::SEGMENT_WIDTH;
  }
}
