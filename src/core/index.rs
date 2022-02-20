use std::{collections::HashMap, io::{Read, Write}, fs::{File, OpenOptions}, str::FromStr};
use camino::{Utf8Path, Utf8PathBuf};
use getset::Getters;
use crate::lib::{*, errors::Errors, constants::{INDEX_PATH, DELETED_INDEX_STAGE}};
use super::{tree::{Tree, Node}, head::Head, blob::Blob};

#[derive(Getters)]
pub struct Index {
  #[getset(get = "pub")]
  staged_paths: HashMap<Utf8PathBuf, String>
}

impl Index {
  pub fn get() -> Result<Self, Errors> {
    let location = locale().join(INDEX_PATH);
    let mut compressed_data = Vec::new();

    File::open(location)?.read_to_end(&mut compressed_data)?;
    let text = if !compressed_data.is_empty() {
      String::from_utf8(decompress(compressed_data)?)?
    } else {
      String::new()
    };

    let mut staged_paths = HashMap::new();
    for line in text.lines() {
      if let [path, stage] = line.split_whitespace().collect::<Vec<&str>>()[..] {
        staged_paths.insert(Utf8PathBuf::from_str(path).unwrap(), String::from(stage));
      }
    }

    Ok(Index { staged_paths })
  }

  pub fn add<P: AsRef<Utf8Path>>(paths: &[P]) -> Result<(), Errors> {
    let mut index = Self::get()?;
    let tree = match Head::get()?.commit() {
      Some(commit) => commit.tree()?,
      None => Tree::new(),
    };

    for path in paths {
      let relative_path = relative(path);

      if relative_path == "" {
        index.stage_folder(path.as_ref(), &tree)?;
      } else {
        match tree.get(relative(path)) {
          Some(Node::Blob(blob)) => index.stage_file(path.as_ref(), blob),
          Some(Node::Tree(tree)) => index.stage_folder(path.as_ref(), tree),
          None => index.stage_untracked(path.as_ref()),
        }?;
      }
    }

    index.save()
  }

  pub fn remove<P: AsRef<Utf8Path>>(paths: &[P]) -> Result<(), Errors> {
    let mut index = Self::get()?;
    let staged_paths = index.staged_paths.clone();

    for path in paths {
      for (staged_path, _) in &staged_paths {
        if staged_path.starts_with(relative(path)) {
          index.staged_paths.remove(staged_path);
        }
      }
    }

    index.save()
  }

  pub fn clear() -> Result<(), Errors> {
    let location = locale().join(INDEX_PATH);
    File::create(location)?.set_len(0)?;

    Ok(())
  }

  // private

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

  fn stage_untracked(&mut self, path: &Utf8Path) -> Result<(), Errors> {
    if !path.exists() {
      return Err(Errors::UnrecognisedPath(Utf8PathBuf::from(path)));
    }

    if path.is_dir() {
      self.insert_folder(path)
    } else {
      self.insert_file(path)
    }
  }

  fn stage_file(&mut self, path: &Utf8Path, blob: &Blob) -> Result<(), Errors> {
    if !path.exists() {
      self.insert_deleted(path);
      return Ok(());
    }

    if path.is_dir() {
      self.insert_folder(path)?;
    } else {
      let blobified_file = Blob::from_path(path)?;

      if blob != &blobified_file {
        self.staged_paths.insert(relative(path), String::from(blobified_file.id()));
      }
    }

    Ok(())
  }

  fn stage_folder(&mut self, path: &Utf8Path, tree: &Tree) -> Result<(), Errors> {
    let blobs = tree.blobs();

    if !path.exists() {
      for (blob_path, _) in &blobs {
        self.insert_deleted(&path.join(blob_path));
      }

      return Ok(());
    }

    if path.is_dir() {
      let file_paths = folder_files(path)?;

      for (blob_path, _) in &blobs {
        let full_blob_path = path.join(blob_path);

        if !file_paths.contains(&full_blob_path) {
          self.insert_deleted(&full_blob_path);
        }
      }

      for file_path in file_paths {
        let relative_file_path = relative(&file_path);
        let blob = blobs.get(&relative_file_path);

        match blob {
          Some(blob) => {
            let blobified_file = Blob::from_path(&file_path)?;

            if blob != &&blobified_file {
              self.staged_paths.insert(relative_file_path, String::from(blobified_file.id()));
            }
          },
          None => {
            self.insert_file(&file_path)?;
          }
        }
      }
    } else {
      self.insert_file(path)?;
    }

    Ok(())
  }

  fn insert_folder(&mut self, path: &Utf8Path) -> Result<(), Errors> {
    let file_paths = folder_files(path)?;

    for file_path in file_paths {
      self.insert_file(&file_path)?;
    }

    Ok(())
  }

  fn insert_file(&mut self, path: &Utf8Path) -> Result<(), Errors> {
    let blob = Blob::from_path(path)?;
    self.staged_paths.insert(relative(path), String::from(blob.id()));

    Ok(())
  }

  fn insert_deleted(&mut self, path: &Utf8Path) {
    self.staged_paths.insert(relative(path), String::from(DELETED_INDEX_STAGE));
  }
}
