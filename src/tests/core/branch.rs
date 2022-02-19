use std::{fs::File, io::Read};
use serial_test::serial;
use crate::{tests::{run_unit, factory::commit}, core::branch::Branch, lib::{constants::BRANCHES_PATH, locale, errors::Errors}};

const TEST_BRANCH_NAME: &str = "test-branch-name";

#[test]
#[serial]
fn new_given_name_and_some_commit_creates_branch() {
  run_unit(|| {
    let commit = commit();
    let branch = Branch::new(TEST_BRANCH_NAME, Some(commit.id())).unwrap();

    let mut commit_id = String::new();
    let location = locale().join(BRANCHES_PATH).join(branch.name());
    File::open(location).unwrap().read_to_string(&mut commit_id).unwrap();

    assert_eq!(&commit_id, commit.id());
  });
}

#[test]
#[serial]
fn new_given_name_and_no_commit_creates_branch() {
  run_unit(|| {
    let branch = Branch::new(TEST_BRANCH_NAME, None).unwrap();

    let mut none = String::new();
    let location = locale().join(BRANCHES_PATH).join(branch.name());
    File::open(location).unwrap().read_to_string(&mut none).unwrap();

    assert!(none.is_empty());
  });
}

#[test]
#[serial]
fn new_given_existing_name_returns_duplicate_branch_error() {
  run_unit(|| {
    Branch::new(TEST_BRANCH_NAME, None).unwrap();

    if let Err(Errors::ExistingBranch(name)) = Branch::new(TEST_BRANCH_NAME, None) {
      assert_eq!(name, TEST_BRANCH_NAME);
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn get_given_name_returns_branch() {
  run_unit(|| {
    let commit = commit();
    Branch::new(TEST_BRANCH_NAME, Some(commit.id())).unwrap();
    let branch = Branch::get(TEST_BRANCH_NAME).unwrap();

    assert_eq!(branch.name(), TEST_BRANCH_NAME);
    assert_eq!(branch.commit_id().as_ref().unwrap(), commit.id());
  });
}

#[test]
#[serial]
fn get_given_non_existing_name_returns_missing_branch_error() {
  run_unit(|| {
    if let Err(Errors::UnrecognisedBranch(name)) = Branch::get(TEST_BRANCH_NAME) {
      assert_eq!(name, TEST_BRANCH_NAME);
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn set_given_name_updates_branch_commit_id() {
  run_unit(|| {
    let commit = commit();
    Branch::new(TEST_BRANCH_NAME, None).unwrap();
    Branch::set(TEST_BRANCH_NAME, commit.id()).unwrap();
    let branch = Branch::get(TEST_BRANCH_NAME).unwrap();

    assert_eq!(branch.name(), TEST_BRANCH_NAME);
    assert_eq!(branch.commit_id().as_ref().unwrap(), commit.id());
  });
}

#[test]
#[serial]
fn set_given_non_existing_name_returns_missing_branch_error() {
  run_unit(|| {
    let commit = commit();
    if let Err(Errors::UnrecognisedBranch(name)) = Branch::set(TEST_BRANCH_NAME, commit.id()) {
      assert_eq!(name, TEST_BRANCH_NAME);
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn remove_given_name_deletes_branch() {
  run_unit(|| {
    Branch::new(TEST_BRANCH_NAME, None).unwrap();
    Branch::remove(TEST_BRANCH_NAME).unwrap();
    let location = locale().join(BRANCHES_PATH).join(TEST_BRANCH_NAME);

    assert!(!location.exists())
  });
}

#[test]
#[serial]
fn remove_given_non_existing_name_returns_missing_branch_error() {
  run_unit(|| {
    if let Err(Errors::UnrecognisedBranch(name)) = Branch::remove(TEST_BRANCH_NAME) {
      assert_eq!(name, TEST_BRANCH_NAME);
    } else {
      unreachable!();
    }
  });
}

#[test]
#[serial]
fn commit_if_present_returns_commit() {
  run_unit(|| {
    let commit = commit();
    let branch = Branch::new(TEST_BRANCH_NAME, Some(commit.id())).unwrap();

    assert_eq!(branch.commit(), Some(commit));
  });
}

#[test]
#[serial]
fn commit_if_not_present_returns_none() {
  run_unit(|| {
    let branch = Branch::new(TEST_BRANCH_NAME, None).unwrap();

    assert_eq!(branch.commit(), None);
  });
}

#[test]
#[serial]
fn initial_branch_file() {
  run_unit(|| {
    let location = locale().join(BRANCHES_PATH).join("master");
    let mut commit_id = String::new();

    File::open(location).unwrap().read_to_string(&mut commit_id).unwrap();

    assert!(commit_id.is_empty());
  });
}
