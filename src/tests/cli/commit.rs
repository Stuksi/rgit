use std::fs::File;
use serial_test::serial;
use crate::{tests::run_acceptance, core::{index::Index, head::Head}, lib::locale};

#[test]
#[serial]
fn commit_does_not_handle_empty_arguments() {
  run_acceptance("commit", |command| {
    command.assert().failure();
  });
}

#[test]
#[serial]
fn commit_does_not_handle_random_options() {
  run_acceptance("commit", |command| {
    command.arg("--option").assert().failure();
  });
}

#[test]
#[serial]
fn commit_does_not_handle_message_option_without_argument() {
  run_acceptance("commit", |command| {
    command.arg("-m").assert().failure();
  });
}

#[test]
#[serial]
fn commit_does_not_handle_message_option_with_multiple_arguments() {
  run_acceptance("commit", |command| {
    command.args(["-m", "commit-message-1", "commit-message-2"]).assert().failure();
  });
}

#[test]
#[serial]
fn commit_handles_message_option_with_single_argument() {
  run_acceptance("commit", |command| {
    command.args(["-m", "commit-message"]).assert().success();
  });
}

#[test]
#[serial]
fn it_outputs_staging_status_on_empty_index() {
  run_acceptance("commit", |command| {
    let output = command.args(["-m", "commit-message"]).output().unwrap().stdout;
    let expected_output_slice = format!("On branch master");

    assert!(output.starts_with(expected_output_slice.as_bytes()));
  })
}

#[test]
#[serial]
fn commit_creates_a_new_commit() {
  run_acceptance("commit", |command| {
    File::create(locale().join("file")).unwrap();
    Index::add(&vec![locale().join("file")]).unwrap();

    let old_commit = Head::get().unwrap().commit();
    command.args(["-m", "commit-message"]).ok().unwrap();
    let new_commit = Head::get().unwrap().commit();

    assert!(old_commit != new_commit);
  });
}
