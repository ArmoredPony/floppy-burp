mod bird;
mod collision;
mod config;
mod pipe;

use bevy::prelude::*;
use bird::BirdPlugin;
use collision::CollisionPlugin;

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Floppy Burp".into(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resolution: Vec2::new(512.0, 1024.0).into(),
            ..default()
          }),
          ..default()
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(CollisionPlugin)
    .add_systems(Startup, setup_game)
    .add_plugins(BirdPlugin)
    .run();
}

fn setup_game(mut commands: Commands) {
  commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
  commands.spawn(Camera2d);
}
