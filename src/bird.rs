use bevy::prelude::*;

use crate::config::*;

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
  query: Query<Entity, With<Bird>>,
) {
  if let Ok(entity) = query.get_single() {
    commands.entity(entity).despawn();
  }
  commands.spawn((
    Sprite {
      image: asset_server.load("bird.png"),
      ..default()
    },
    Bird::default(),
    Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
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
  if keys.just_pressed(KeyCode::Space) {
    bird.velocity = FLAP_FORCE;
  }
  bird.velocity -= time.delta_secs() * GRAVITY_COEF;
  transform.translation.y += bird.velocity * time.delta_secs();
  transform.rotation = Quat::from_axis_angle(
    Vec3::Z,
    f32::clamp(bird.velocity / VEL_TO_ANGLE_RATIO, -90.0, 90.0).to_radians(),
  );
}
