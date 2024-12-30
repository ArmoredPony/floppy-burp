use bevy::prelude::App;
use floppy_burp::FloppyBurpPlugin;

fn main() {
  App::new().add_plugins(FloppyBurpPlugin).run();
}
