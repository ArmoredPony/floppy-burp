use bevy::{
  math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
  prelude::*,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PostUpdate, debug_shapes);
  }
}

#[derive(Component)]
pub enum Shape {
  Rectangle(Rectangle),
  Circle(Circle),
}

impl Shape {
  pub fn to_collider(&self, center: Vec2) -> Collider {
    match self {
      Shape::Rectangle(r) => {
        Collider::Rectangle(Aabb2d::new(center, r.half_size))
      }
      Shape::Circle(c) => {
        Collider::Circle(BoundingCircle::new(center, c.radius))
      }
    }
  }
}

pub enum Collider {
  Rectangle(Aabb2d),
  Circle(BoundingCircle),
}

impl Collider {
  pub fn collides(&self, other: &Collider) -> bool {
    match self {
      Collider::Rectangle(aabb2d) => match other {
        Collider::Rectangle(other_aabb2d) => aabb2d.intersects(other_aabb2d),
        Collider::Circle(other_bounding_circle) => {
          aabb2d.intersects(other_bounding_circle)
        }
      },
      Collider::Circle(bounding_circle) => match other {
        Collider::Rectangle(other_aabb2d) => {
          bounding_circle.intersects(other_aabb2d)
        }
        Collider::Circle(other_bounding_circle) => {
          bounding_circle.intersects(other_bounding_circle)
        }
      },
    }
  }
}

fn debug_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
  for (shape, transform) in &query {
    let isometry = Isometry2d::from(transform.translation.xy());
    match shape {
      Shape::Rectangle(r) => {
        gizmos.primitive_2d(r, isometry, Color::srgb(1.0, 0.0, 0.0));
      }
      Shape::Circle(c) => {
        gizmos.primitive_2d(c, isometry, Color::srgb(0.0, 0.0, 1.0));
      }
    }
  }
}
