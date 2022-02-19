use crate::{lib::{errors::Errors, constants::DELETED_INDEX_STAGE, object::FromId}, core::{head::{Head, Reference}, tree::{Tree, Node}, index::Index, blob::Blob, commit::Commit, branch::Branch}};
use super::status::status;

pub fn commit(message: String) -> Result<(), Errors> {
  let index = Index::get()?;

  if index.staged_paths().is_empty() {
    return status();
  }

  let head = Head::get()?;
  let parent_commit = head.commit();
  let mut tree = match &parent_commit {
    Some(commit) => commit.tree()?,
    None => Tree::new()
  };

  for (path, stage) in index.staged_paths() {
    if stage == DELETED_INDEX_STAGE {
      tree.remove(path);
    } else {
      tree.insert(path, Node::Blob(Blob::from_id(stage)?));
    }
  }

  let parent_commit_id = parent_commit.map(|commit| String::from(commit.id()));
  let commit = Commit::new(parent_commit_id.as_deref(), tree.pack()?, &message)?;

  match head.reference() {
    Reference::Branch(branch) => Branch::set(branch.name(), commit.id()),
    Reference::Commit(_) => Head::set(&Reference::Commit(commit)),
  }?;

  Index::clear()?;

  Ok(())
}
