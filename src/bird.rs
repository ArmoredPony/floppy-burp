use std::ops::Add;

use bevy::prelude::*;

use crate::{
  collision::Shape,
  layer::Layer,
  pipe::Pipe,
  state::GameState,
  RESOLUTION,
};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Idle), respawn_bird)
      .add_systems(
        OnTransition {
          exited: GameState::GameOver,
          entered: GameState::Going,
        },
        respawn_bird,
      )
      .add_systems(
        Update,
        (update_bird, detect_collision).run_if(in_state(GameState::Going)),
      );
  }
}

#[derive(Component, Default)]
pub struct Bird {
  pub velocity: f32,
}

impl Bird {
  pub const FLAP_FORCE: f32 = 120.0;
  pub const GRAVITY_COEF: f32 = 700.0;
  pub const VEL_TO_ANGLE_RATIO: f32 = 8.0;
  pub const HITBOX_SIZE: f32 = 4.0;
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
    Bird::default(),
    Sprite::from_image(asset_server.load("bird.png")),
    Transform::from_xyz(-RESOLUTION.x / 4.0, 0.0, Layer::Bird.into()),
    Shape::Circle(Circle::new(Bird::HITBOX_SIZE)),
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
    bird.velocity = Bird::FLAP_FORCE;
  } else {
    bird.velocity -= time.delta_secs() * Bird::GRAVITY_COEF;
  }
  transform.translation.y = transform
    .translation
    .y
    .add(bird.velocity * time.delta_secs())
    .clamp(
      -RESOLUTION.y / 2.0 + Bird::HITBOX_SIZE,
      RESOLUTION.y / 2.0 + Bird::HITBOX_SIZE * 2.0,
    );
  transform.rotation = Quat::from_axis_angle(
    Vec3::Z,
    f32::clamp(bird.velocity / Bird::VEL_TO_ANGLE_RATIO, -90.0, 90.0)
      .to_radians(),
  );
}

fn detect_collision(
  mut next_state: ResMut<NextState<GameState>>,
  bird_query: Single<(&Shape, &Transform), With<Bird>>,
  obstacle_query: Query<(&Shape, &Transform), With<Pipe>>,
) {
  let bird_collider = bird_query.0.to_collider(bird_query.1.translation.xy());
  for obstacle in &obstacle_query {
    if bird_collider
      .collides(&obstacle.0.to_collider(obstacle.1.translation.xy()))
    {
      next_state.set(GameState::GameOver);
    }
  }
}
