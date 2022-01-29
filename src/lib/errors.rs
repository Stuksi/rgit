#[derive(Debug)]
pub enum Errors {
  UnknownObjectError(String),
  BadObjectStructureError,
  ExistingRepositoryError,
  InvalidTreeNodeError,
  NodeConvertionError,
  InvalidUsernameOrEmail,
  DuplicateBranchNameError(String),
  MissingBranchError(String),
  IOError(std::io::Error),
  Utf8ConvertionError(std::string::FromUtf8Error),
  DateTimeParseError(chrono::ParseError),
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
