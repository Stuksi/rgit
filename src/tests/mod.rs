mod core;
mod lib;
mod gens;

use std::env;
use tempdir::TempDir;
use crate::lib::{constants::PROJECT_ENV, initialize};

const TEMP_DIR_PREFIX: &str = "rust-git-test";

pub fn run_unit(body: fn()) {
  let temp_dir = TempDir::new(TEMP_DIR_PREFIX).unwrap();
  env::set_var(PROJECT_ENV, temp_dir.path());
  initialize().unwrap();
  body();
}
