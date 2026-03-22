use std::collections::HashMap;
use std::fmt::{Display, Error};

pub type ShadeUKey = u32;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum ShadeKey {
  U(ShadeUKey),
  Default
}

impl Display for ShadeKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
    match self {
      ShadeKey::U(shade_key) => write!(f, "{}", shade_key),
      ShadeKey::Default => write!(f, "DEFAULT")
    }
  }
}

pub fn is_valid_hex(hex: &str) -> bool {
  hex.starts_with("#") && hex.chars().skip(1).all(|c| c.is_digit(16))
}
pub fn shades_of(hex: &str) -> Result<HashMap<ShadeKey, String>, String> {
  if !is_valid_hex(hex) {
    return Err(format!("Invalid hex: {}", hex));
  }
  let base_color = hex_to_rgb_array(hex);
  let black: Vec<u8> = vec![0, 0, 0];
  let white: Vec<u8> = vec![255, 255, 255];

  let shades: Vec<ShadeUKey> = vec![50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

  let mut result = HashMap::new();
  if !result.contains_key(&ShadeKey::Default) {
    result.insert(ShadeKey::Default, hex.to_string());
  }

  for shade in shades.iter() {
    if *shade == 500 {
      result.insert(ShadeKey::U(500), hex.to_string());
    } else {
      if *shade < 500 {
        let percentage = (500 - shade) as f32 / 500.0;
        result.insert(ShadeKey::U(*shade), get_color(percentage, white.clone(), base_color.clone()));
      } else {
        let percentage = (shade - 500) as f32 / 500.0;
        result.insert(ShadeKey::U(*shade), get_color(percentage, base_color.clone(), black.clone()));
      }
    }
  }

  Ok(result)
}

fn hex_to_rgb_array(hex: &str) -> Vec<u8> {
  let mut hex = String::from(hex);

  hex = hex.replace("#", "");

  if hex.len() == 3 {
    hex = format!("{}{}", hex, hex);
  }

  let (r, gb) = hex.split_at(2);
  let (g, b) = gb.split_at(2);

  let vec_channels = vec![r.to_string(), g.to_string(), b.to_string()];

  let vec_int_channels: Vec<u8> = vec_channels.iter().map(|channel| {
    u8::from_str_radix(channel, 16).unwrap()
  }).collect::<Vec<_>>();

  vec_int_channels
}

fn get_color(percentage: f32, start: Vec<u8>, end: Vec<u8>) -> String {
  let rgb = end.iter().enumerate().map(|(index, channel)| {
    let rgb_channel = (*channel as f32) + percentage * (start[index] - channel) as f32;
    rgb_channel.round()
  });

  let hex_channel = rgb.map(|ch| {
    format!("{:02x}", ch as u8)
  }).collect::<String>();

  format!("#{}", hex_channel)
}