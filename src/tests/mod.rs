mod core;
mod lib;
mod cli;

use std::env;
use assert_cmd::Command;
use tempfile::TempDir;
use crate::lib::{constants::PROJECT_ENV, initialize};

const TEMP_DIR_PREFIX: &str = "rgit-test";

pub fn run_unit(body: fn()) {
  let temp_dir = TempDir::new().unwrap();
  env::set_var(PROJECT_ENV, temp_dir.path());
  initialize().unwrap();

  body();
}

pub fn run_acceptance(name: &str, body: fn(command: &mut Command) -> ()) {
  let temp_dir = TempDir::new().unwrap();
  env::set_var(PROJECT_ENV, temp_dir.path());
  initialize().unwrap();

  let mut command = Command::cargo_bin("rgit").unwrap();
  command.current_dir(temp_dir.path());
  command.arg(name);
  body(&mut command);
}

mod factory {
  use lipsum::lipsum;
  use crate::core::{blob::Blob, tree::{Tree, Node}, commit::Commit, branch::Branch};

  pub const TREE_PATHS: &[&str] = &[
    "a/a/a", "a/a/b", "a/a/c",
    "a/b/a", "a/b/b", "a/b/c",
    "a/c/a", "a/c/b", "a/c/c",
    "b/a/a", "b/a/b", "b/a/c",
    "b/b/a", "b/b/b", "b/b/c",
    "b/c/a", "b/c/b", "b/c/c",
    "c/a/a", "c/a/b", "c/a/c",
    "c/b/a", "c/b/b", "c/b/c",
    "c/c/a", "c/c/b", "c/c/c",
  ];

  pub fn blob() -> Blob {
    Blob::new(lipsum(100).as_bytes()).unwrap()
  }

  pub fn blob_and_text() -> (Blob, String) {
    let text = lipsum(25);
    (Blob::new(text.as_bytes()).unwrap(), text)
  }

  pub fn tree() -> Tree {
    let mut tree = Tree::new();

    for path in TREE_PATHS {
      tree.insert(path, Node::Blob(blob()));
    }

    tree.pack().unwrap();
    tree
  }

  pub fn commit() -> Commit {
    Commit::new(None, tree().id(), &lipsum(1)).unwrap()
  }

  pub fn branch() -> Branch {
    Branch::new(&lipsum(25), Some(commit().id())).unwrap()
  }
}
