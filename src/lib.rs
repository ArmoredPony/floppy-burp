#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod background;
mod bird;
mod checkpoint;
mod collision;
mod ground;
mod layer;
mod pipe;
mod score;
mod state;
mod ui;

use background::BackgroundPlugin;
use bevy::{
  log::LogPlugin,
  window::{PresentMode, WindowResolution},
};
use bird::BirdPlugin;
use checkpoint::CheckpointPlugin;
use collision::CollisionPlugin;
use ground::GroundPlugin;
use pipe::PipePlugin;
use score::ScorePlugin;
use state::{GameState, GameStatePlugin};
use ui::UiPlugin;

const GAME_SPEED: f32 = 100.0;
const PHYSICAL_RESOLUTION: Vec2 = Vec2::new(360.0, 720.0);
const SCALE_FACTOR: f32 = 2.0;
const RESOLUTION: Vec2 = Vec2::new(
  PHYSICAL_RESOLUTION.x / SCALE_FACTOR,
  PHYSICAL_RESOLUTION.y / SCALE_FACTOR,
);

pub struct FloppyBurpPlugin;

impl Plugin for FloppyBurpPlugin {
  fn build(&self, app: &mut App) {
    let default_plugins = DefaultPlugins
      .set(WindowPlugin {
        primary_window: Some(Window {
          title: "Floppy Burp".into(),
          position: WindowPosition::Centered(MonitorSelection::Primary),
          resolution: WindowResolution::from(PHYSICAL_RESOLUTION)
            .with_scale_factor_override(SCALE_FACTOR),
          present_mode: PresentMode::AutoVsync,
          resizable: false,
          ..default()
        }),
        ..default()
      })
      .set(ImagePlugin::default_nearest());
    #[cfg(debug_assertions)]
    let default_plugins = default_plugins.set(LogPlugin {
      filter: "floppy_burp=debug".into(),
      ..default()
    });
    app
      .add_plugins((
        default_plugins,
        BackgroundPlugin,
        GameStatePlugin,
        CollisionPlugin,
        GroundPlugin,
        BirdPlugin,
        PipePlugin,
        CheckpointPlugin,
        ScorePlugin,
        UiPlugin,
      ))
      .insert_state(GameState::Idle)
      .add_systems(Startup, setup_game);
  }
}

fn setup_game(mut commands: Commands) {
  commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
  commands.spawn(Camera2d);
}
