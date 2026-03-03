use std::process::Command;
use crate::types::metadata::ComponentMetadata;
use crate::utils::constants::get_repo_config;

pub fn fetch_metadata(component_name: &str) -> Result<ComponentMetadata, Box<dyn std::error::Error>> {
  let ( repo_owner, repo_name, repo_branch, raw_github_url) = get_repo_config();

  let url = format!("{}/{}/{}/{}/src/components/{}/component.toml",
    raw_github_url, repo_owner, repo_name, repo_branch, component_name
  );
  let output = Command::new("curl")
    .arg("-s")
    .arg(url)
    .output()?;

  if !output.status.success() {
    return Err(format!("Failed to fetch metadata for component '{}': {}", component_name, String::from_utf8(output.stderr)?).into());
  }
  if output.stdout.is_empty() {
    return Err(format!("No metadata found for component '{}'", component_name).into());
  }

  let metadata = String::from_utf8(output.stdout)?;
  let metadata: ComponentMetadata = toml::from_str(&metadata)?;
  Ok(metadata)
}