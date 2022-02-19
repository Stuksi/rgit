use chrono::{DateTime, Utc};
use getset::Getters;
use crate::lib::{errors::Errors, user::User, object::{Object, FromId}, write_object_bytes, read_object_bytes, constants::OBJECTS_PATH, locale};
use super::tree::Tree;

#[derive(Getters, Debug, PartialEq, Clone)]
pub struct Commit {
  #[getset(get = "pub")]
  id: String,

  #[getset(get = "pub")]
  parent_id: Option<String>,

  #[getset(get = "pub")]
  tree_id: String,

  #[getset(get = "pub")]
  message: String,

  #[getset(get = "pub")]
  date: DateTime<Utc>,

  #[getset(get = "pub")]
  author: User,
}

impl Commit {
  pub fn new(parent_id: Option<&str>, tree_id: &str, message: &str) -> Result<Self, Errors> {
    let tree_location = locale().join(OBJECTS_PATH).join(&tree_id[..2]).join(&tree_id[2..]);

    if !tree_location.exists() {
      return Err(Errors::UnrecognisedObject(String::from(tree_id)));
    }

    let date = Utc::now();
    let author = User::get()?;
    let bytes = format!("{}\n{}\n{}\n{}\n{}\n{}",
      parent_id.unwrap_or("0"),
      tree_id,
      message,
      date.to_rfc3339(),
      author.username(),
      author.email(),
    );
    let id = write_object_bytes(Object::Commit, bytes)?;

    Ok(
      Commit {
        id,
        parent_id: parent_id.map(|id| String::from(id)),
        tree_id: String::from(tree_id),
        message: String::from(message),
        date,
        author: User::get()?,
      }
    )
  }

  pub fn tree(&self) -> Result<Tree, Errors> {
    Tree::from_id(self.tree_id())
  }

  pub fn parent(&self) -> Option<Commit> {
    match self.parent_id() {
      Some(id) => {
        let parent = Commit::from_id(&id);
        if let Ok(commit) = parent {
          Some(commit)
        } else {
          None
        }
      },
      None => None,
    }
  }
}

impl FromId for Commit {
  fn from_id(id: &str) -> Result<Self, Errors> {
    let bytes = read_object_bytes(id)?;
    let text = String::from_utf8(bytes)?;

    if let [
      parent_id,
      tree_id,
      message,
      date,
      username,
      email,
    ] = text.lines().collect::<Vec<&str>>()[..] {
      let parent_id = if parent_id == "0" {
        None
      } else {
        Some(String::from(parent_id))
      };

      Ok(
        Commit {
          id: String::from(id),
          parent_id,
          tree_id: String::from(tree_id),
          message: String::from(message),
          date: DateTime::parse_from_rfc3339(date)?.with_timezone(&Utc),
          author: User::new(username, email)?,
        }
      )
    } else {
      Err(Errors::BadObjectStructure)
    }
  }
}
