mod bird;
mod checkpoint;
mod collision;
mod layer;
mod pipe;
mod state;

use bevy::{prelude::*, window::WindowResolution};
use bird::BirdPlugin;
use checkpoint::CheckpointPlugin;
use collision::CollisionPlugin;
use pipe::PipePlugin;
use state::{GameState, GameStatePlugin};

const GAME_SPEED: f32 = 40.0;
const PHYSICAL_RESOLUTION: Vec2 = Vec2::new(360.0, 720.0);
const SCALE_FACTOR: f32 = 5.0;
const RESOLUTION: Vec2 = Vec2::new(
  PHYSICAL_RESOLUTION.x / SCALE_FACTOR,
  PHYSICAL_RESOLUTION.y / SCALE_FACTOR,
);

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Floppy Burp".into(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resolution: WindowResolution::from(PHYSICAL_RESOLUTION)
              .with_scale_factor_override(5.0),
            ..default()
          }),
          ..default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .insert_state(GameState::Idle)
    .add_systems(Startup, setup_game)
    .add_plugins(GameStatePlugin)
    .add_plugins(CollisionPlugin)
    .add_plugins(PipePlugin)
    .add_plugins(BirdPlugin)
    .add_plugins(CheckpointPlugin)
    .run();
}

fn setup_game(mut commands: Commands) {
  commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
  commands.spawn(Camera2d);
}
