use std::ops::Add;

use bevy::prelude::*;

use crate::{
  collision::Shape,
  ground::Ground,
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
      )
      .add_systems(
        FixedUpdate,
        control_bird.run_if(in_state(GameState::Going)),
      );
  }
}

#[derive(Component)]
pub struct Bird;

#[derive(Component, Deref, Clone, Copy, Default, Debug)]
pub struct Velocity(f32);

impl Bird {
  pub const FLAP_FORCE: f32 = 300.0;
  pub const GRAVITY_COEF: f32 = 1800.0;
  pub const VEL_TO_ANGLE_RATIO: f32 = 8.0;
  pub const HITBOX_SIZE: f32 = 10.0;
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
    Bird,
    Velocity::default(),
    Sprite::from_image(asset_server.load("yellowbird-midflap.png")),
    Transform::from_xyz(-RESOLUTION.x / 4.0, 0.0, Layer::Bird.into()),
    Shape::Circle(Circle::new(Bird::HITBOX_SIZE)),
  ));
}

fn update_bird(
  time: Res<Time>,
  query: Single<(&mut Velocity, &mut Transform), With<Bird>>,
) {
  let (mut velocity, mut transform) = query.into_inner();
  velocity.0 -= time.delta_secs() * Bird::GRAVITY_COEF;
  transform.translation.y = transform
    .translation
    .y
    .add(velocity.0 * time.delta_secs())
    .clamp(
      -RESOLUTION.y / 2.0 + Bird::HITBOX_SIZE + Ground::LEVEL,
      RESOLUTION.y / 2.0 + Bird::HITBOX_SIZE * 2.0,
    );
  transform.rotation = Quat::from_axis_angle(
    Vec3::Z,
    f32::clamp(velocity.0 / Bird::VEL_TO_ANGLE_RATIO, -90.0, 90.0).to_radians(),
  );
}

fn control_bird(
  keys: Res<ButtonInput<KeyCode>>,
  mut velocity: Single<&mut Velocity, With<Bird>>,
) {
  if keys.pressed(KeyCode::Space) {
    velocity.0 = Bird::FLAP_FORCE;
  }
}

fn detect_collision(
  mut next_state: ResMut<NextState<GameState>>,
  bird_query: Single<(&Shape, &Transform), With<Bird>>,
  obstacle_query: Query<(&Shape, &Transform), Or<(With<Pipe>, With<Ground>)>>,
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
