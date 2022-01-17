mod core;

use std::env;
use tempdir::TempDir;
use crate::lib::constants::PROJECT_ENV;

pub fn run_unit(body: fn()) {
  let temp_dir = TempDir::new("rust-git-test").unwrap();
  env::set_var(PROJECT_ENV, temp_dir.path());
  body();
}
