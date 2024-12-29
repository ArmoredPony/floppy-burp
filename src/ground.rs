use bevy::prelude::*;

use crate::{collision::Shape, layer::Layer, RESOLUTION};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, init_ground);
  }
}

#[derive(Component)]
pub struct Ground;

impl Ground {
  pub const LEVEL: f32 = 50.0;
}

fn init_ground(mut commands: Commands) {
  commands.spawn((
    Ground,
    Transform::from_xyz(
      0.0,
      (-RESOLUTION.y + Ground::LEVEL) / 2.0,
      Layer::Ground.into(),
    ),
    Shape::Rectangle(Rectangle::new(RESOLUTION.x, Ground::LEVEL)),
  ));
}
