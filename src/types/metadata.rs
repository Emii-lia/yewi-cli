use serde::Deserialize;

#[derive(Deserialize)]
pub struct ComponentMetadata {
  pub name: String,
  #[serde(default = "Vec::new")]
  pub dependencies: Vec<String>,
}