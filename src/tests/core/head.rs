use std::{fs::File, io::Read};
use serial_test::serial;
use crate::{tests::{run_unit, factory::{commit, branch}}, core::{head::{Head, Reference}, branch::Branch}, lib::{locale, constants::HEAD_PATH}};

#[test]
#[serial]
fn get_given_master_branch_returns_head() {
  run_unit(|| {
    let head = Head::get().unwrap();
    let branch = Branch::get("master").unwrap();

    assert_eq!(head.reference(), &Reference::Branch(branch));
  });
}

#[test]
#[serial]
fn get_given_commit_returns_head() {
  run_unit(|| {
    let commit = commit();
    Head::set(&Reference::Commit(commit.clone())).unwrap();
    let head = Head::get().unwrap();

    assert_eq!(head.reference(), &Reference::Commit(commit));
  });
}

#[test]
#[serial]
fn set_given_branch_reference_updates_head() {
  run_unit(|| {
    let branch = branch();
    Head::set(&Reference::Branch(branch.clone())).unwrap();
    let head = Head::get().unwrap();

    assert_eq!(head.reference(), &Reference::Branch(branch));
  });
}

#[test]
#[serial]
fn set_given_commit_reference_updates_head() {
  run_unit(|| {
    let commit = commit();
    Head::set(&Reference::Commit(commit.clone())).unwrap();
    let head = Head::get().unwrap();

    assert_eq!(head.reference(), &Reference::Commit(commit));
  });
}

#[test]
#[serial]
fn commit_returns_reference_branch_commit() {
  run_unit(|| {
    let branch = branch();
    Head::set(&Reference::Branch(branch.clone())).unwrap();
    let head = Head::get().unwrap();

    assert_eq!(head.commit(), branch.commit());
  });
}

#[test]
#[serial]
fn commit_returns_reference_commit_commit() {
  run_unit(|| {
    let commit = commit();
    Head::set(&Reference::Commit(commit.clone())).unwrap();
    let head = Head::get().unwrap();

    assert_eq!(head.commit(), Some(commit));
  });
}

#[test]
#[serial]
fn initial_head_file() {
  run_unit(|| {
    let location = locale().join(HEAD_PATH);
    let mut reference = String::new();

    File::open(location).unwrap().read_to_string(&mut reference).unwrap();

    assert_eq!(reference, "master");
  });
}

#[test]
#[serial]
fn detached_head_file() {
  run_unit(|| {
    let commit = commit();
    Head::set(&Reference::Commit(commit.clone())).unwrap();

    let location = locale().join(HEAD_PATH);
    let mut reference = String::new();

    File::open(location).unwrap().read_to_string(&mut reference).unwrap();

    assert_eq!(reference, format!("detached:{}", commit.id()));
  });
}
