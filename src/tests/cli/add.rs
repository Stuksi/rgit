use serial_test::serial;
use crate::tests::run_acceptance;

#[test]
#[serial]
fn it_does_not_handle_empty_arguments() {
  run_acceptance("add", |command| {
    command.assert().failure();
  });
}

#[test]
#[serial]
fn it_does_not_handle_options() {
  run_acceptance("add", |command| {
    command.arg("--option").assert().failure();
  });
}

#[test]
#[serial]
fn it_handles_multiple_arguments() {
  run_acceptance("add", |command| {
    command.args(["a/a", "a/a/a"]).assert().success();
  });
}
