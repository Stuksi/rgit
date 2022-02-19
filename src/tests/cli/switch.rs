use serial_test::serial;
use crate::{tests::{run_acceptance, factory::{commit, branch}}, core::head::{Head, Reference}};

#[test]
#[serial]
fn it_does_not_handle_empty_arguments() {
  run_acceptance("switch", |command| {
    command.assert().failure();
  });
}

#[test]
#[serial]
fn it_does_not_handle_random_options() {
  run_acceptance("switch", |command| {
    command.arg("--option").assert().failure();
  });
}

#[test]
#[serial]
fn it_does_not_handle_multiple_arguments() {
  run_acceptance("switch", |command| {
    command.args(["argument-1", "argument-2"]).assert().failure();
  });
}

#[test]
#[serial]
fn it_handles_new_option() {
  run_acceptance("switch", |command| {
    command.args(["-n", "test-branch"]).assert().success();
  });
}

#[test]
#[serial]
fn it_handles_commit_option() {
  run_acceptance("switch", |command| {
    command.args(["-c", "commit-id"]).assert().success();
  });
}

#[test]
#[serial]
fn it_does_not_handle_commit_and_new_options_at_the_same_time() {
  run_acceptance("switch", |command| {
    command.arg("-nc").assert().failure();
  });
}

#[test]
#[serial]
fn it_handles_single_argument() {
  run_acceptance("switch", |command| {
    command.arg("branch-name").assert().success();
  });
}

// #[test]
// #[serial]
// fn it_prints_invalid_branch_messages() {
//   run_acceptance("switch", |command| {
//     let output = command.args(["-n", "master"]).output().unwrap().stdout;
//     let expected_output = format!("error: branch master already exists\n");

//     assert_eq!(output, expected_output.as_bytes());
//   })
// }

#[test]
#[serial]
fn it_creates_a_new_branch() {
  run_acceptance("switch", |command| {
    let commit = commit();
    Head::set(&Reference::Commit(commit.clone())).unwrap();

    command.args(["-n", "test-branch"]).ok().unwrap();

    match Head::get().unwrap().reference() {
      Reference::Branch(branch) => {
        assert_eq!(branch.name(), "test-branch");
        assert_eq!(branch.commit(), Some(commit));
      },
      _ => unreachable!()
    };
  });
}

#[test]
#[serial]
fn it_changes_to_a_commit_in_headless_mode() {
  run_acceptance("switch", |command| {
    let commit = commit();

    command.args(["-c", commit.id()]).ok().unwrap();

    match Head::get().unwrap().reference() {
      Reference::Commit(ref_commit) => {
        assert_eq!(ref_commit, &commit);
      },
      _ => unreachable!()
    };
  });
}

#[test]
#[serial]
fn it_changes_to_a_branch() {
  run_acceptance("switch", |command| {
    let branch = branch();

    command.arg(branch.name()).ok().unwrap();

    match Head::get().unwrap().reference() {
      Reference::Branch(ref_branch) => {
        assert_eq!(ref_branch, &branch);
      },
      _ => unreachable!()
    };
  });
}
