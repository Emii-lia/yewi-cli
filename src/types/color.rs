use std::fmt::{Display, Error, Formatter};

pub enum Color {
  Slate,
  Gray,
  Zinc,
  Neutral,
  Stone,
  Emerald,
  Blue,
  Sky,
  Custom(String),
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
      Color::Custom(String::from("Custom")),
    ].into_iter()
  }
  pub fn get_colors() -> Vec<Color> {
    Color::iter().collect()
  }
  pub fn from_str(s: &str) -> Color {
    match s.to_lowercase().as_str() {
      "slate" => Color::Slate,
      "gray" => Color::Gray,
      "zinc" => Color::Zinc,
      "neutral" => Color::Neutral,
      "stone" => Color::Stone,
      "emerald" => Color::Emerald,
      "blue" => Color::Blue,
      "sky" => Color::Sky,
      "custom" => Color::Custom(String::from("Custom")),
      _ => Color::Custom(s.to_string()),
    }
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
      Color::Custom(_s) => write!(f, "Custom"),
    }
  }
}