use std::env;
use crate::lib::{errors::Errors, initialize, constants::{PROJECT_ENV, REPOSITORY_PATH}};

pub fn init() -> Result<(), Errors> {
  let current_dir = env::current_dir()?;

  if current_dir.join(REPOSITORY_PATH).exists() {
    return Err(Errors::ExistingRepository);
  }

  env::set_var(PROJECT_ENV, current_dir);
  initialize()?;

  Ok(())
}
