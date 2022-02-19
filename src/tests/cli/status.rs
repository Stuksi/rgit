use std::fs;
use serial_test::serial;
use crate::{tests::{run_acceptance, factory::{TREE_PATHS, tree, commit}}, core::{index::Index, head::{Head, Reference}}, lib::locale};

#[test]
#[serial]
fn status_does_not_handle_any_arguments() {
  run_acceptance("status", |command| {
    command.arg("argument").assert().failure();
  });
}

#[test]
#[serial]
fn status_does_not_handle_any_options() {
  run_acceptance("status", |command| {
    command.arg("--option").assert().failure();
  });
}

#[test]
#[serial]
fn status_handles_default_call() {
  run_acceptance("status", |command| {
    command.assert().success();
  });
}

#[test]
#[serial]
fn status_prints_current_working_tree_status() {
  run_acceptance("status", |command| {
    tree().unpack(locale()).unwrap();

    let commit = commit();
    Head::set(&Reference::Commit(commit.clone())).unwrap();

    fs::remove_file(locale().join("c/c/c")).unwrap();
    Index::add(&vec![locale().join("a/")]).unwrap();

    let output = command.output().unwrap().stdout;
    let mut expected_output = String::new();

    expected_output += &format!("\
HEAD detached at {}
Changes to be committed:
  (use \"rgit restore <PATHS>...\" to unstage)
", commit.id());

    for file_path in &TREE_PATHS[..9] {
      expected_output +=  &format!("      modified:  {}\n", file_path);
    }

    expected_output += &format!("\n\
Changes not staged for commit:
  (use \"rgit add <PATHS>...\" to update what will be committed)
");

    expected_output +=  &format!("      deleted:   c/c/c\n");

    for file_path in &TREE_PATHS[9..26] {
      expected_output +=  &format!("      modified:  {}\n", file_path);
    }

    expected_output += "\n";

    assert_eq!(output, expected_output.as_bytes());
  });
}
