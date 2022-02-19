pub mod init;
pub mod add;
pub mod restore;
pub mod commit;
pub mod switch;
pub mod config;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
  #[clap(about = "Creates an empty repository")]
  Init,

  #[clap(about = "Stages files for commit")]
  Add {
    files: Vec<String>,
  },

  #[clap(about = "Unstages staged files")]
  Restore {
    files: Vec<String>,
  },

  #[clap(about = "Creates a snapshot of the changes")]
  Commit {
    #[clap(short, long)]
    message: String,
  },

  #[clap(about = "Switches to a different snapshot")]
  Switch {
    #[clap(short, long, conflicts_with = "commit", help = "Creates a new branch")]
    new: bool,

    #[clap(short, long, conflicts_with = "new", help = "Switches to a commit in detached mode")]
    commit: bool,

    #[clap(help = "Branch name or commit hash")]
    target: String,
  },

  #[clap(about = "Sets the user info")]
  Config {
    #[clap(long, required_unless_present = "email")]
    username: Option<String>,

    #[clap(long, required_unless_present = "email")]
    email: Option<String>,
  },
}