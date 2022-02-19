use std::fs;
use serial_test::serial;
use crate::{tests::run_acceptance, lib::{constants::*, locale}};

#[test]
#[serial]
fn init_given_argument_fails() {
  run_acceptance("init", |command| {
    command.arg("ARGUMENT").assert().failure();
  });
}

#[test]
#[serial]
fn init_given_option_fails() {
  run_acceptance("init", |command| {
    command.arg("--option").assert().failure();
  });
}

#[test]
#[serial]
fn init_given_no_arguments_and_options_succeeds() {
  run_acceptance("init", |command| {
    fs::remove_dir_all(locale().join(REPOSITORY_PATH)).unwrap();
    command.assert().success();
  });
}

#[test]
#[serial]
fn init_if_repository_exists_fails_with_error() {
  run_acceptance("init", |command| {
    let output = command.output().unwrap().stdout;
    let expected_output = "error: rgit repository already instantiated in current working environment\n";

    assert_eq!(output, expected_output.as_bytes());
  });
}

#[test]
#[serial]
fn init_if_no_repository_exists_succeeds_creating_repository() {
  run_acceptance("init", |command| {
    let locale = locale();
    fs::remove_dir_all(locale.join(REPOSITORY_PATH)).unwrap();
    command.ok().unwrap();

    assert!(locale.join(REPOSITORY_PATH).exists());
    assert!(locale.join(OBJECTS_PATH).exists());
    assert!(locale.join(BRANCHES_PATH).exists());
    assert!(locale.join(HEAD_PATH).exists());
    assert!(locale.join(INDEX_PATH).exists());
    assert!(locale.join(CONFIG_PATH).exists());
  });
}
