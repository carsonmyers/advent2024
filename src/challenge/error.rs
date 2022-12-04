use thiserror::Error;

use crate::input;

#[derive(Error, Debug, Default)]
pub enum Error {
    #[error("invalid day `{0}`")]
    InvalidDay(usize),
    #[error("day `{0}` not implemented")]
    DayNotImplemented(usize),
    #[error("input error: {0}")]
    InputError(#[from] input::error::Error),
    #[error("error parsing int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("missing data in challenge: {0}")]
    MissingDataError(String),
    #[error("invalid command in challenge: {0}")]
    InvalidCommandError(String),
    #[error("too many lines of input")]
    TooManyLinesError(),
    #[error("no solution found")]
    NoSolutionError(),
    #[error("unknown error")]
    #[default]
    UnknownError,
}

impl Error {
    pub fn missing_data(name: &str) -> Self {
        Error::MissingDataError(name.to_string())
    }
}
