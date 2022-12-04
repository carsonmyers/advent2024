use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("path error")]
    PathError,
    #[error("parse error: {0}")]
    ParseError(#[from] url::ParseError),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("file `{0}` is not utf8")]
    Utf8Error(PathBuf),
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::PathError
    }
}
