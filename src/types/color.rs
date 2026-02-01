use std::fmt::{Display, Error, Formatter};

pub enum Color {
  Slate,
  Gray,
  Zinc,
  Neutral,
  Stone,
  Emerald,
  Blue,
  Sky
}

impl Color {
  pub fn iter() -> impl Iterator<Item = Color> {
    vec![
      Color::Slate,
      Color::Gray,
      Color::Zinc,
      Color::Neutral,
      Color::Stone,
      Color::Emerald,
      Color::Blue,
      Color::Sky,
    ].into_iter()
  }
  pub fn get_colors() -> Vec<Color> {
    Color::iter().collect()
  }
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    match self {
      Color::Slate => write!(f, "Slate"),
      Color::Gray => write!(f, "Gray"),
      Color::Zinc => write!(f, "Zinc"),
      Color::Neutral => write!(f, "Neutral"),
      Color::Stone => write!(f, "Stone"),
      Color::Emerald => write!(f, "Emerald"),
      Color::Blue => write!(f, "Blue"),
      Color::Sky => write!(f, "Sky"),
    }
  }
}