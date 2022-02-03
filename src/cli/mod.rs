mod commands;

use std::env;
use clap::Parser;
use crate::lib::{errors::Errors, constants::PROJECT_ENV};
use commands::init::init;
use self::commands::{Commands, add::add, restore::restore, commit::commit, switch::switch, config::config};

#[derive(Parser)]
#[clap(name = "rgit")]
#[clap(author, version)]
#[clap(about = "Version control system powered by rust")]
struct Interface {
  #[clap(subcommand)]
  commands: Commands,
}

pub fn run() -> Result<(), Errors> {
  let interface = Interface::parse();

  if let Commands::Init = interface.commands {
    return init();
  }

  // find existing repository

  if let Err(_) = env::var(PROJECT_ENV) {
    return Err(Errors::NotARepositoryError);
  }

  match interface.commands {
    Commands::Add { files } => add(files),
    Commands::Restore { files } => restore(files),
    Commands::Commit { message } => commit(message),
    Commands::Switch { new, commit, target } => switch(new, commit, target),
    Commands::Config { username, email } => config(username, email),
    _ => Ok(()),
  }
}
