use crate::{core::{head::{Head, Reference}, tree::Tree, index::Index, blob::Blob}, lib::{folder_files, locale, errors::Errors, constants::DELETED_INDEX_STAGE, relative}};

pub fn status() -> Result<(), Errors> {
  let head = Head::get()?;
  let commit = head.commit();
  let tree = match commit {
    Some(commit) => commit.tree()?,
    None => Tree::new(),
  };
  let index = Index::get()?;
  let blobs = tree.blobs();
  let file_paths = folder_files(locale())?;
  let mut status = String::new();

  status += &match head.reference() {
    Reference::Branch(branch) => format!("On branch {}\n", branch.name()),
    Reference::Commit(commit) => format!("HEAD detached at {}\n", commit.id())
  };

  let mut changes_staged_for_commit = Vec::new();
  let mut changes_not_staged_for_commit = Vec::new();

  if !index.staged_paths().is_empty() {
    for (path, stage) in index.staged_paths() {
      changes_staged_for_commit.push(if stage == DELETED_INDEX_STAGE  {
        format!("      deleted:   {}\n", path)
      } else {
        format!("      modified:  {}\n", path)
      });
    }
  }

  for (blob_path, _) in &blobs {
    let full_blob_path = locale().join(blob_path);

    if !file_paths.contains(&full_blob_path) && !index.staged_paths().contains_key(blob_path) {
      changes_not_staged_for_commit.push(format!("      deleted:   {}\n", blob_path));
    }
  }

  for file_path in file_paths {
    if !index.staged_paths().contains_key(&relative(&file_path)) {
      let relative_file_path = relative(&file_path);
      let blob = blobs.get(&relative_file_path);

      match blob {
        Some(blob) => {
          let blobified_file = Blob::from_path(&file_path)?;

          if blob != &&blobified_file {
            changes_not_staged_for_commit.push(format!("      modified:  {}\n", relative_file_path));
          }
        },
        None => {
          changes_not_staged_for_commit.push(format!("      modified:  {}\n", relative(file_path)));
        }
      }
    }
  }

  changes_staged_for_commit.sort();
  changes_not_staged_for_commit.sort();

  if !changes_staged_for_commit.is_empty() {
    status += "Changes to be committed:\n";
    status += "  (use \"rgit restore <PATHS>...\" to unstage)\n";

    for change in &changes_staged_for_commit {
      status += &change;
    }

    status += "\n";
  }

  if !changes_not_staged_for_commit.is_empty() {
    status += "Changes not staged for commit:\n";
    status += "  (use \"rgit add <PATHS>...\" to update what will be committed)\n";

    for change in &changes_not_staged_for_commit {
      status += &change;
    }

    status += "\n";
  }

  if changes_staged_for_commit.is_empty() && changes_not_staged_for_commit.is_empty() {
    status += "nothing to commit, working tree clean\n";
  }

  print!("{}", status);

  Ok(())
}
