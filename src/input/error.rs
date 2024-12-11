use std::path::PathBuf;
use std::sync::PoisonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("no session file: place advent of code session key in ./.session")]
    NoSessionFile,
    #[error("input file {0} does not exist")]
    NoInputFile(PathBuf),
    #[error("http error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("path error")]
    PathError,
    #[error("url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("input panicked: {0}")]
    InputPanicError(String),
    #[error("poisoned input lock")]
    PoisonedInputLockError,
    #[error("line parse error: {0}")]
    LineParseError(String),
    #[error("file `{0}` is not utf8")]
    Utf8Error(PathBuf),
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::PathError
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Error::PoisonedInputLockError
    }
}
