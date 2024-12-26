mod bird;
mod collision;
mod layer;
mod pipe;

use bevy::{prelude::*, window::WindowResolution};
use bird::BirdPlugin;
use collision::CollisionPlugin;
use pipe::PipePlugin;

const GAME_SPEED: f32 = 40.0;

fn main() {
  App::new()
    .add_systems(
      FixedUpdate,
      |mut commands: Commands,
       focused_windows: Query<Entity, With<Window>>,
       input: Res<ButtonInput<KeyCode>>| {
        for window in focused_windows.iter() {
          if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
          }
        }
      },
    )
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Floppy Burp".into(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resolution: WindowResolution::new(360.0, 720.0)
              .with_scale_factor_override(3.0),
            ..default()
          }),
          ..default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_systems(Startup, setup_game)
    .add_plugins(CollisionPlugin)
    .add_plugins(PipePlugin)
    .add_plugins(BirdPlugin)
    .run();
}

fn setup_game(mut commands: Commands) {
  commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
  commands.spawn(Camera2d);
}
