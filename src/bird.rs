use bevy::prelude::*;

use crate::{collision::Shape, config::*};

pub const FLAP_FORCE: f32 = 700.0;
pub const GRAVITY_COEF: f32 = 2000.0;
pub const VEL_TO_ANGLE_RATIO: f32 = 8.0;
pub const HITBOX_SIZE: f32 = 20.0;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, respawn_bird)
      .add_systems(Update, update_bird);
  }
}

#[derive(Component, Default)]
struct Bird {
  pub velocity: f32,
}

fn respawn_bird(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  window: Query<&Window>,
  query: Query<Entity, With<Bird>>,
) {
  if let Ok(entity) = query.get_single() {
    commands.entity(entity).despawn();
  }
  let window = window.single();
  commands.spawn((
    Sprite::from_image(asset_server.load("bird.png")),
    Bird::default(),
    Transform::from_translation(Vec3::new(-window.width() / 4.0, 0.0, 0.0))
      .with_scale(Vec3::splat(PIXEL_RATIO)),
    Shape::Circle(Circle::new(HITBOX_SIZE)),
  ));
}

fn update_bird(
  time: Res<Time>,
  keys: Res<ButtonInput<KeyCode>>,
  mut query: Query<(&mut Bird, &mut Transform)>,
) {
  let Ok((mut bird, mut transform)) = query.get_single_mut() else {
    return;
  };
  if keys.pressed(KeyCode::Space) {
    bird.velocity = FLAP_FORCE;
  }
  bird.velocity -= time.delta_secs() * GRAVITY_COEF;
  transform.translation.y += bird.velocity * time.delta_secs();
  transform.rotation = Quat::from_axis_angle(
    Vec3::Z,
    f32::clamp(bird.velocity / VEL_TO_ANGLE_RATIO, -90.0, 90.0).to_radians(),
  );
}