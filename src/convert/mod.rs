use std::collections::BTreeMap;
use crate::convert::colorize::colorize;
use crate::utils::shade::{shades_of, ShadeKey};

pub mod colorize;
pub mod test;

pub fn convert(hex: String, colored: Option<bool>) -> Result<BTreeMap<ShadeKey, String>, String> {
  let shade = shades_of(hex.as_str());
  match shade {
    Ok(sh) => {
      if colored.unwrap_or(false) {
        let mut colorized_shades = BTreeMap::new();
        for (key, value) in sh.clone() {
          let fg = match key {
            ShadeKey::U(num) => {
              if num < 500 {
                "black"
              } else {
                "white"
              }
            }
            ShadeKey::Default => {
              "white"
            }
          };
          let colorized = colorize(value.as_str(), fg, value.as_str());
          if !colorized_shades.contains_key(&key) {
            colorized_shades.insert(key, colorized);
          }
        }
        
        Ok(colorized_shades)
      } else {
        Ok(sh)
      }
    }
    Err(_) => {
      return Err("Failed to convert hex to Shade".to_string());
    }
  }
}