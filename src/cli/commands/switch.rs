use crate::{lib::{errors::Errors, object::FromId, locale}, core::{head::{Head, Reference}, commit::Commit, branch::Branch}, cli::clear_project};

pub fn switch(new: bool, commit: bool, target: String) -> Result<(), Errors> {
  let head = Head::get()?;
  let commit_id = head.commit().map(|commit| String::from(commit.id()));
  let reference;

  if new {
    reference = Reference::Branch(Branch::new(&target, commit_id.as_deref())?);
  } else if commit {
    reference = Reference::Commit(Commit::from_id(&target)?);
  } else {
    reference = Reference::Branch(Branch::get(&target)?);
  }

  clear_project()?;

  Head::set(&reference)?;
  match reference {
    Reference::Branch(branch) => {
      if let Some(commit) = branch.commit() {
        commit.tree()?.unpack(locale())?;
      }
    },
    Reference::Commit(commit) => {
      commit.tree()?.unpack(locale())?;
    }
  }

  Ok(())
}
