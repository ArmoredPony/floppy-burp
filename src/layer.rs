#[repr(i16)]
pub enum Layer {
  Background,
  Ground,
  Pipe,
  Bird,
}

impl From<Layer> for i16 {
  fn from(value: Layer) -> Self {
    value as i16
  }
}

impl From<Layer> for f32 {
  fn from(value: Layer) -> Self {
    i16::from(value).into()
  }
}
