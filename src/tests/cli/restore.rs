use serial_test::serial;
use crate::tests::run_acceptance;

#[test]
#[serial]
fn restore_does_not_handle_empty_arguments() {
  run_acceptance("restore", |command| {
    command.assert().failure();
  });
}

#[test]
#[serial]
fn restore_does_not_handle_options() {
  run_acceptance("restore", |command| {
    command.arg("--option").assert().failure();
  });
}

#[test]
#[serial]
fn restore_handles_multiple_arguments() {
  run_acceptance("restore", |command| {
    command.args(["a/a", "a/a/a"]).assert().success();
  });
}
