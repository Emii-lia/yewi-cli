use std::env::temp_dir;
use std::fs;
use std::fs::remove_dir_all;
use std::path::PathBuf;
use crate::init::create;
use crate::utils::shade::{shades_of, ShadeKey};

fn temp_project() -> (PathBuf, PathBuf) {
  let dir = temp_dir();
  let project = dir.join("yew-test-project");
  (dir, project)
}

#[test]
fn create_fails_if_dir_already_exists() {
  let (_dir, project) = temp_project();
  fs::create_dir_all(&project).unwrap();

  let result = create(
    project.to_str().unwrap(),
    Some("slate".into()),
    Some(true)
  );
  assert!(result.is_err());
  assert!(result.unwrap_err().to_string().contains("already exists"));
}

#[test]
fn create_rejects_invalid_theme() {
  let project = temp_dir().join("yew-test-invalid-theme");
  let result = create(
    project.to_str().unwrap(),
    Some("invalid-color".into()),
    Some(true)
  );

  assert!(result.is_err());
  assert!(result.unwrap_err().to_string().contains("Invalid hex color"));
  assert!(!project.exists());
}

#[test]
fn produces_expected_project_structure() {
  let project = temp_dir().join("yew-test-structure");

  create(
    project.to_str().unwrap(),
    Some("sky".into()),
    Some(true)
  ).unwrap();

  assert!(project.join("src").exists());
  assert!(project.join("src").join("components").exists());
  assert!(project.join("Cargo.toml").exists());
  assert!(project.join("package.json").exists());
  assert!(project.join("index.html").exists());
  assert!(project.join("public").exists());
  assert!(project.join("Trunk.toml").exists());
  assert!(project.join("tailwind.config.js").exists());
  assert!(project.join("postcss.config.js").exists());
  assert!(project.join("src").join("styles").exists());
  assert!(project.join("src").join("styles").join("main.scss").exists());
  assert!(project.join("src").join("app").exists());
  assert!(project.join("src").join("app").join("mod.rs").exists());
  assert!(project.join("src").join("app").join("page.rs").exists());
  assert!(project.join("src").join("app").join("routes.rs").exists());
  assert!(remove_dir_all(&project).is_ok());
}

#[test]
fn create_with_i18n_produces_expected_project_structure() {
  let project = temp_dir().join("yew-test-i18n-structure");

  create(
    project.to_str().unwrap(),
    Some("sky".into()),
    Some(true)
  ).unwrap();

  assert!(project.join("src").exists());
  assert!(project.join("src").join("components").exists());
  assert!(project.join("Cargo.toml").exists());
  assert!(project.join("package.json").exists());
  assert!(project.join("index.html").exists());
  assert!(project.join("public").exists());
  assert!(project.join("Trunk.toml").exists());
  assert!(project.join("tailwind.config.js").exists());
  assert!(project.join("postcss.config.js").exists());
  assert!(project.join("src").join("styles").exists());
  assert!(project.join("src").join("styles").join("main.scss").exists());
  assert!(project.join("src").join("app").exists());
  assert!(project.join("src").join("app").join("mod.rs").exists());
  assert!(project.join("src").join("app").join("page.rs").exists());
  assert!(project.join("src").join("app").join("routes.rs").exists());
  assert!(project.join("src").join("i18n").exists());
  assert!(project.join("src").join("i18n").join("en").join("base.json").exists());

  let cargo_toml = fs::read_to_string(project.join("Cargo.toml")).unwrap();
  assert!(cargo_toml.contains("i18nrs"));

  assert!(remove_dir_all(&project).is_ok());
}

#[test]
fn create_with_custom_hex_theme() {
  let project = temp_dir().join("yew-test-custom-theme");
  let hex = "#eb9868";
  create(
    project.to_str().unwrap(),
    Some(hex.into()),
    Some(true)
  ).unwrap();

  let shades = shades_of(hex).unwrap();

  assert!(project.join("src").join("styles").join("main.scss").exists());
  let style = fs::read_to_string(project.join("src").join("styles").join("main.scss")).unwrap();

  for (shade, value) in shades {
    match shade {
      ShadeKey::U(key) => {
        assert!(style.contains(&format!("--primary-{}: {}", key, value)));
      }
      ShadeKey::Default => {
        assert!(style.contains(&format!("--primary: {}", value)));
      }
    }
  }
  assert!(remove_dir_all(&project).is_ok());
}