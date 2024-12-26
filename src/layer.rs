pub enum Layer {
  Bird = 0,
  Pipe = -1,
  Background = i16::MIN as isize,
}

impl From<Layer> for i32 {
  fn from(value: Layer) -> Self {
    value as i32
  }
}

impl From<Layer> for f32 {
  fn from(value: Layer) -> Self {
    value as i32 as f32
  }
}
