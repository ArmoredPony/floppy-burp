use bevy::{prelude::*, sprite::Anchor};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, debug_bbox);
  }
}

#[derive(Component)]
pub enum Shape {
  Rectangle(Rectangle),
  Circle(Circle),
}

fn debug_bbox(
  mut gizmos: Gizmos,
  query: Query<(&Shape, &Transform, Option<&Anchor>)>,
) {
  for (shape, transform, anchor) in &query {
    let isometry = Isometry2d::from(
      transform.translation.xy()
        * anchor.map(|a| a.as_vec() * 2.0).unwrap_or(Vec2::ONE),
    );
    let color = Color::srgb(1.0, 0.0, 0.0);
    match shape {
      Shape::Rectangle(r) => {
        gizmos.primitive_2d(r, isometry, color);
      }
      Shape::Circle(c) => {
        gizmos.primitive_2d(c, isometry, color);
      }
    }
  }
}
