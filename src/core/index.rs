use std::{collections::HashMap, io::{Read, Write}, fs::{File, OpenOptions}};
use camino::Utf8Path;
use getset::Getters;
use crate::lib::{errors::Errors, locale, constants::INDEX_PATH, locale_relative, compress, decompress};
use super::{tree::{Tree, Node}, head::Head, blob::Blob};

#[derive(Getters)]
pub struct Index {
  #[getset(get = "pub")]
  staged_paths: HashMap<String, String>
}

impl Index {
  pub fn add<P: AsRef<Utf8Path>>(paths: &[P]) -> Result<(), Errors> {
    let mut index = Self::get()?;
    let tree = match Head::get()?.commit() {
      Some(commit) => commit.tree()?,
      None => Tree::new(),
    };

    for path in paths {
      if !path.as_ref().exists() {
        return Err(Errors::UnrecognisedPath(path.as_ref().to_string()));
      }

      index.diff_and_insert(path, &tree)?;
    }

    index.save()
  }

  pub fn remove<P: AsRef<Utf8Path>>(paths: &[P]) -> Result<(), Errors> {
    let mut index = Self::get()?;

    for path in paths {
      let relative_path = locale_relative(&path);
      index.staged_paths.remove(relative_path.as_str());
    }

    index.save()
  }

  // private

  fn get() -> Result<Self, Errors> {
    let location = locale().join(INDEX_PATH);
    let mut compressed_data = Vec::new();

    File::open(location)?.read_to_end(&mut compressed_data)?;
    let text = if compressed_data.is_empty() {
      String::new()
    } else {
      String::from_utf8(decompress(compressed_data)?)?
    };

    let mut staged_paths = HashMap::new();
    for line in text.lines() {
      if let [relative_path, id] = line.split_whitespace().collect::<Vec<&str>>()[..] {
        staged_paths.insert(String::from(relative_path), String::from(id));
      }
    }

    Ok(
      Index {
        staged_paths
      }
    )
  }

  fn save(&self) -> Result<(), Errors> {
    let location = locale().join(INDEX_PATH);
    let mut data = String::new();

    for (relative_path, id) in &self.staged_paths {
      data += &format!("{} {}\n", relative_path, id);
    }

    let compressed_data = if data != "" {
      compress(data.trim_end().as_bytes())?
    } else {
      Vec::new()
    };

    OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(location)?
      .write_all(&compressed_data)?;

    Ok(())
  }

  fn diff_and_insert<P: AsRef<Utf8Path>>(&mut self, path: P, tree: &Tree) -> Result<(), Errors> {
    let mut bytes = Vec::new();
    File::open(path.as_ref())?.read_to_end(&mut bytes)?;

    let blob = Blob::new(bytes)?;
    let relative_path = locale_relative(&path);

    if let Some(Node::Blob(tree_blob)) = tree.get(&relative_path) {
      if tree_blob.id() == blob.id() {
        return Ok(());
      }
    }

    self.staged_paths.insert(String::from(relative_path), String::from(blob.id()));
    Ok(())
  }
}
