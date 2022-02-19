use camino::Utf8PathBuf;

#[derive(Debug)]
pub enum Errors {
  MissingRepository,
  ExistingRepository,
  ExistingBranch(String),
  BadPathEncoding,
  UnrecognisedPath(Utf8PathBuf),
  UnrecognisedObject(String),
  UnrecognisedBranch(String),

  BadNodeConvertion,
  BadObjectStructure,
  UnrecognisedNodeType,
  IOError(std::io::Error),
  Utf8ConvertionError(std::string::FromUtf8Error),
  DateTimeParseError(chrono::ParseError),
  WalkDirError(walkdir::Error),
}

impl From<std::io::Error> for Errors {
  fn from(error: std::io::Error) -> Self {
    Self::IOError(error)
  }
}

impl From<std::string::FromUtf8Error> for Errors {
  fn from(error: std::string::FromUtf8Error) -> Self {
    Self::Utf8ConvertionError(error)
  }
}

impl From<chrono::ParseError> for Errors {
  fn from(error: chrono::ParseError) -> Self {
    Self::DateTimeParseError(error)
  }
}

impl From<walkdir::Error> for Errors {
  fn from(error: walkdir::Error) -> Self {
    Self::WalkDirError(error)
  }
}

pub struct ErrorsInterface;

impl ErrorsInterface {
  pub fn handle(error: Errors) {
    match error {
      Errors::MissingRepository => println!("error: not a rgit repository (or any of the parent directories)"),
      Errors::ExistingRepository => println!("error: rgit repository already instantiated in current working environment"),
      Errors::ExistingBranch(name) => println!("error: a branch named '{}' already exists", name),
      Errors::BadPathEncoding => println!("error: bad path encoding (only utf8 is supported)"),
      Errors::UnrecognisedPath(path) => println!("error: path '{}' did not match any files", path),
      Errors::UnrecognisedObject(id) => println!("error: object identificator '{}' did not match any object", id),
      Errors::UnrecognisedBranch(name) => println!("error: branch name '{}' does not exist", name),
      _ => print!("fatal: Internal Error")
    };
  }
}
