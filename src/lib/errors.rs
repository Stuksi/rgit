#[derive(Debug)]
pub enum Errors {
  UnknownObjectError(String),
  BadObjectStructureError,
  BadFilePath,
  ExistingRepositoryError,
  InvalidTreeNodeError,
  NodeConvertionError,
  NotARepositoryError,
  BadUTF8PathError,
  UnrecognisedPath(String),
  DuplicateBranchNameError(String),
  MissingBranchError(String),
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
      Errors::ExistingRepositoryError => print!("error: rgit repository already exists in current directory\n"),
      _ => println!("OK"),
    }
  }
}
