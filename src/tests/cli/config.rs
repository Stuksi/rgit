use serial_test::serial;
use crate::{tests::run_acceptance, lib::user::User};

#[test]
#[serial]
fn config_does_not_handle_any_arguments() {
  run_acceptance("config", |command| {
    command.arg("argument").assert().failure();
  });
}

#[test]
#[serial]
fn config_does_not_handle_empty_options() {
  run_acceptance("config", |command| {
    command.assert().failure();
  });
}

#[test]
#[serial]
fn config_handles_single_username_option() {
  run_acceptance("config", |command| {
    command.arg("--username=username").assert().success();
  });
}

#[test]
#[serial]
fn config_handles_single_email_option() {
  run_acceptance("config", |command| {
    command.arg("--email=email").assert().success();
  });
}

#[test]
#[serial]
fn config_handles_both_options() {
  run_acceptance("config", |command| {
    command.args(["--username=username", "--email=email"]).assert().success();
  });
}

#[test]
#[serial]
fn config_updates_the_user_credentials() {
  run_acceptance("config", |command| {
    command.args(["--username=username", "--email=email"]).ok().unwrap();

    let user = User::get().unwrap();

    assert_eq!(user.username(), "username");
    assert_eq!(user.email(), "email");
  });
}
