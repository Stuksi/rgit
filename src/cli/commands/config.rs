use crate::lib::{errors::Errors, user::User};

pub fn config(username: Option<String>, email: Option<String>) -> Result<(), Errors> {
  if let Some(username) = username {
    User::set_username(&username)?;
  }

  if let Some(email) = email {
    User::set_email(&email)?;
  }

  Ok(())
}
