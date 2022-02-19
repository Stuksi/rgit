use serial_test::serial;
use crate::{lib::user::User, tests::run_unit};

const USERNAME: &str = "Jeremy";
const EMAIL: &str = "jeremy@mail.com";
const ADMIN: &str = "admin";

#[test]
#[serial]
fn new_given_username_and_email_creates_user() {
  run_unit(|| {
    let user = User::new(USERNAME, EMAIL).unwrap();

    assert_eq!(user.username(), USERNAME);
    assert_eq!(user.email(), EMAIL);
  });
}

#[test]
#[serial]
fn get_returns_the_stored_user() {
  run_unit(|| {
    let user = User::get().unwrap();

    assert_eq!(user.username(), ADMIN);
    assert_eq!(user.email(), ADMIN);
  });
}

#[test]
#[serial]
fn set_username_given_username_changes_the_current_user_username() {
  run_unit(|| {
    User::set_username(USERNAME).unwrap();
    let user = User::get().unwrap();

    assert_eq!(user.username(), USERNAME);
    assert_eq!(user.email(), ADMIN);
  });
}

#[test]
#[serial]
fn set_email_given_email_changes_the_current_user_email() {
  run_unit(|| {
    User::set_email(EMAIL).unwrap();
    let user = User::get().unwrap();

    assert_eq!(user.username(), ADMIN);
    assert_eq!(user.email(), EMAIL);
  });
}
