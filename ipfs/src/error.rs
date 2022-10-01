use std::io;

#[derive(Debug)]
pub enum Error {
  IpfsError,
  JsonError(serde_json::Error),
  IoError(io::Error),
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::IoError(err)
  }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Error {
    Error::JsonError(err)
  }
}
