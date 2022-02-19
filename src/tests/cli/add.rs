use serial_test::serial;
use crate::{tests::run_acceptance, lib::locale};

#[test]
#[serial]
fn add_does_not_handle_empty_arguments() {
  run_acceptance("add", |command| {
    command.assert().failure();
  });
}

#[test]
#[serial]
fn add_does_not_handle_options() {
  run_acceptance("add", |command| {
    command.arg("--option").assert().failure();
  });
}

#[test]
#[serial]
fn add_handles_multiple_arguments() {
  run_acceptance("add", |command| {
    command.args(["a/a", "a/a/a"]).assert().success();
  });
}

#[test]
#[serial]
fn add_outputs_unrecognized_path_on_no_matching_path() {
  run_acceptance("add", |command| {
    let output = command.args(["d"]).output().unwrap().stdout;
    let expected_output = format!("error: path '{}' did not match any files\n", locale().join("d"));

    assert_eq!(output, expected_output.as_bytes());
  });
}
