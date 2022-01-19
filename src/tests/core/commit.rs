use std::{fs::File, io::Read};
use serial_test::serial;
use crate::{tests::{run_unit, gens::tree}, core::commit::Commit, lib::{locale, constants::OBJECTS_PATH, decompress, errors::Errors, object::FromId}};

#[test]
#[serial]
fn new_given_parent_commit_and_tree_creates_commit() {
  run_unit(|| {
    let tree = tree();
    let commit = Commit::new(None, tree.id()).unwrap();

    let mut compressed = Vec::new();
    let location = locale().join(OBJECTS_PATH).join(&commit.id()[..2]).join(&commit.id()[2..]);
    File::open(location).unwrap().read_to_end(&mut compressed).unwrap();

    let expected = format!("cmit{} {} {} {} {}",
      "0",
      tree.id(),
      commit.date().to_rfc3339(),
      "admin",
      "admin",
    );

    assert!(!commit.id().is_empty());
    assert_eq!(decompress(&compressed).unwrap(), expected.as_bytes());
  });
}

#[test]
#[serial]
fn new_given_invalid_tree_returns_error() {
  run_unit(|| {
    if let Err(Errors::UnknownObjectError(id)) = Commit::new(None, "invalid_id") {
      assert_eq!(id, "invalid_id");
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn tree_returns_the_associated_tree() {
  run_unit(|| {
    let tree = tree();
    let commit = Commit::new(None, tree.id()).unwrap();

    assert_eq!(commit.tree().unwrap(), tree);
  });
}

#[test]
#[serial]
fn parent_returns_the_associated_parent_commit() {
  run_unit(|| {
    let parent = Commit::new(None, tree().id()).unwrap();
    let commit = Commit::new(Some(parent.id()), tree().id()).unwrap();

    assert_eq!(commit.parent().unwrap(), parent);
  });
}

#[test]
#[serial]
fn parent_returns_none_if_the_parent_id_is_invalid() {
  run_unit(|| {
    let commit = Commit::new(Some("invalid_id"), tree().id()).unwrap();

    assert_eq!(commit.parent(), None);
  });
}

#[test]
#[serial]
fn from_id_given_existing_id_returns_commit() {
  run_unit(|| {
    let commit = Commit::new(None, tree().id()).unwrap();
    let from_commit = <Commit as FromId>::from(commit.id()).unwrap();

    assert_eq!(commit, from_commit);
  });
}
