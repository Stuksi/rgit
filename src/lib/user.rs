use std::{fs::{File, OpenOptions}, io::{Read, Write}};
use getset::Getters;
use crate::lib::errors::Errors;
use super::{locale, constants::CONFIG_PATH};

#[derive(Getters, Debug, PartialEq, Clone)]
pub struct User {
  #[getset(get = "pub")]
  username: String,

  #[getset(get = "pub")]
  email: String,
}

impl User {
  pub fn new(username: &str, email: &str) -> Result<Self, Errors> {
    Ok(
      User {
        username: String::from(username),
        email: String::from(email),
      }
    )
  }

  pub fn get() -> Result<Self, Errors> {
    let location = locale().join(CONFIG_PATH);
    let mut config = String::new();

    File::open(location)?.read_to_string(&mut config)?;

    if let [username, email] = config.split_whitespace().collect::<Vec<&str>>()[..] {
      Ok(
        User {
          username: String::from(username),
          email: String::from(email),
        }
      )
    } else {
      return Err(Errors::BadObjectStructure);
    }
  }

  pub fn set_username(username: &str) -> Result<(), Errors> {
    let user = User::get()?;
    let location = locale().join(CONFIG_PATH);
    let config = format!("{} {}", username, user.email());

    OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(location)?
      .write_all(config.as_bytes())?;

    Ok(())
  }

  pub fn set_email(email: &str) -> Result<(), Errors> {
    let user = User::get()?;
    let location = locale().join(CONFIG_PATH);
    let config = format!("{} {}", user.username(), email);

    OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(location)?
      .write_all(config.as_bytes())?;

    Ok(())
  }
}
