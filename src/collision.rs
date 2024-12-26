use bevy::{
  math::bounding::{Aabb2d, BoundingCircle, BoundingVolume},
  prelude::*,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(FixedUpdate, update_colliders)
      .add_systems(PostUpdate, debug_bbox);
  }
}

#[derive(Component)]
pub enum Collider {
  Rectangle(Aabb2d),
  Circle(BoundingCircle),
}

impl Collider {
  pub fn center(&self) -> Vec2 {
    match self {
      Collider::Rectangle(aabb2d) => aabb2d.center(),
      Collider::Circle(bounding_circle) => bounding_circle.center(),
    }
  }
}

fn update_colliders(mut query: Query<(&mut Collider, &Transform)>) {
  for (collider, transform) in &mut query {
    // dbg!(transform.translation.xy(), collider.center());
    match collider.into_inner() {
      Collider::Rectangle(mut aabb2d) => {
        aabb2d.translate_by(transform.translation.xy());
        dbg!(aabb2d.center());
      }
      Collider::Circle(mut bounding_circle) => {
        bounding_circle.translate_by(transform.translation.xy());
      }
    }
  }
}

fn debug_bbox(mut gizmos: Gizmos, query: Query<&Collider>) {
  for collider in &query {
    let isometry = Isometry2d::from(collider.center());
    let color = Color::srgb(1.0, 0.0, 0.0);
    match collider {
      Collider::Rectangle(r) => {
        gizmos.primitive_2d(
          &Rectangle::from_size(r.half_size()),
          isometry,
          color,
        );
      }
      Collider::Circle(c) => {
        gizmos.primitive_2d(&Circle::new(c.radius()), isometry, color);
      }
    }
  }
}
