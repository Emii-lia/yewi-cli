use std::error::Error;
use clap::{Parser, Subcommand};
use crate::add::add;
use crate::init::create;

mod add;
mod utils;
mod types;
mod handlers;
mod init;

#[derive(Subcommand)]
enum Commands {
  #[command(
    alias = "create",
    about = "Create a new Yewi project",
    long_about = "Create a new Yewi project with all pre-configured settings and dependencies. Alias: 'create'",
    override_usage = "yewi new <project_name>",
  )]
  New {
    project_name: String
  },
  #[command(
    aliases = &["install", "i"],
    about = "Add one or more Yewi components to an existing project.",
    long_about = "Add one or more Yewi components to an existing project. You can specify multiple component names. Aliases: 'install', 'i'",
    override_usage = "yewi add <component_name1> <component_name2> ...",
  )]
  Add {
    component_names: Vec<String>
  }
}
#[derive(Parser)]
#[command(name = "yewi")]
#[command(version, about, author = "Emii-lia")]
#[command(next_line_help = true)]
#[command(
  about = "Yewi CLI - A tool to manage Yewi components",
  long_about = None,
  override_usage = "\n\n yewi new <project_name>\n yewi add <component_name1> <component_name2> ..."
)]
struct Cli {
  #[command(subcommand)]
  command: Commands
}

fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::parse();

  match cli.command {
    Commands::New { project_name } => {
      create(&project_name)?;
    }
    Commands::Add { component_names } => {
      for component_name in component_names {
        add(&component_name)?;
      };
    }
  }

  Ok(())
}
