use std::fmt::{Display, Error, Formatter};

pub enum NodePackageMan {
  NPM(&'static [&'static str]),
  YARN(&'static [&'static str]),
  PNPM(&'static [&'static str]),
  BUN(&'static [&'static str]),
}

impl Default for NodePackageMan {
  fn default() -> Self {
    NodePackageMan::NPM(&["run", "build"])
  }
}

impl NodePackageMan {
  pub fn get_build_command(&self) -> &'static [&'static str] {
    match self {
      NodePackageMan::NPM(cmds) => cmds,
      NodePackageMan::YARN(cmds) => cmds,
      NodePackageMan::PNPM(cmds) => cmds,
      NodePackageMan::BUN(cmds) => cmds,
    }
  }
  pub fn yarn() -> Self {
    NodePackageMan::YARN(&["build"])
  }
  pub fn pnpm() -> Self {
    NodePackageMan::PNPM(&["run", "build"])
  }
  pub fn bun() -> Self {
    NodePackageMan::BUN(&["run", "build"])
  }
  pub fn from_str(s: &str) -> Option<Self> {
    match s.to_lowercase().as_str() {
      "npm" => Some(NodePackageMan::NPM(&["run", "build"])),
      "yarn" => Some(NodePackageMan::YARN(&["build"])),
      "pnpm" => Some(NodePackageMan::PNPM(&["run", "build"])),
      "bun" => Some(NodePackageMan::BUN(&["run", "build"])),
      _ => return None,
    }
  }
  pub fn to_string(&self) -> String {
    match self {
      NodePackageMan::NPM(_) => { "npm".to_string() }
      NodePackageMan::YARN(_) => { "yarn".to_string() }
      NodePackageMan::PNPM(_) => { "pnpm".to_string() }
      NodePackageMan::BUN(_) => { "bun".to_string() }
    }
  }
  pub fn iter() -> impl Iterator<Item = NodePackageMan> {
    vec![
      NodePackageMan::NPM(&["run", "build"]),
      NodePackageMan::YARN(&["build"]),
      NodePackageMan::PNPM(&["run", "build"]),
      NodePackageMan::BUN(&["run", "build"]),
    ].into_iter()
  }
  pub fn get_packages() -> Vec<NodePackageMan> {
    NodePackageMan::iter().collect()
  }
}

impl Display for NodePackageMan {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    match self {
      NodePackageMan::NPM(_) => { write!(f, "npm") },
      NodePackageMan::YARN(_) => { write!(f, "yarn")}
      NodePackageMan::PNPM(_) => { write!(f, "pnpm") }
      NodePackageMan::BUN(_) => { write!(f, "bun") }
    }
  }
}