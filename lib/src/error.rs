#[derive(Debug)]
pub enum Error {
  IO(std::io::Error),
  Parse,
}

impl std::error::Error for Error { }

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::IO(e) => e.fmt(f),
      Error::Parse => f.write_str("Parse error"),
    }
  }
}

impl std::cmp::PartialEq for Error {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Error::IO(_), Error::IO(_)) =>
        false,
      (Error::Parse, Error::Parse) =>
        true,
      _ =>
        false,
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Error {
    Error::IO(e)
  }
}

impl From<std::num::ParseIntError> for Error {
  fn from(_: std::num::ParseIntError) -> Error {
    Error::Parse
  }
}

impl From<lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>,
  &str>> for Error
{
  fn from(_: lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>,
              &str>) -> Error
  {
    Error::Parse
  }
}

