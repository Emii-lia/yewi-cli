use crate::convert::convert;

#[test]
fn convert_invalid_hex() {
  let hex = "#invalid-hex";
  let result = convert(hex.to_string(), None);
  assert!(result.is_err());
  assert!(result.unwrap_err().to_string().contains("Failed to convert hex to Shade"));
}

#[test]
fn convert_valid_hex() {
  let hex = "#ff0000";
  let result = convert(hex.to_string(), None);
  assert!(result.is_ok());
  let shades = result.unwrap();
  assert_eq!(shades.len(), 12);
}

#[test]
fn convert_valid_hex_with_colorized_output() {
  let hex = "#ff0000";
  let result = convert(hex.to_string(), Some(true));
  assert!(result.is_ok());
  let shades = result.unwrap();
  assert_eq!(shades.len(), 12);
}

#[test]
fn convert_hex_without_prefix() {
  let hex = "ff0000";
  let result = convert(hex.to_string(), None);
  assert!(result.is_ok());
  let shades = result.unwrap();
  assert_eq!(shades.len(), 12);
}

#[test]
fn convert_hex_with_3_digit_color() {
  let hex = "#f00";
  let result = convert(hex.to_string(), None);
  assert!(result.is_ok());
  let shades = result.unwrap();
  assert_eq!(shades.len(), 12);
}