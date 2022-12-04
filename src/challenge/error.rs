use thiserror::Error;

use crate::input;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("invalid day `{0}`")]
    InvalidDay(usize),
    #[error("day `{0}` not implemented")]
    DayNotImplemented(usize),
    #[error("input error: {0}")]
    InputError(#[from] input::error::Error),
    #[error("error parsing int `{1}`: {0}")]
    ParseIntError(std::num::ParseIntError, String),
}

#[derive(Error, Debug)]
#[error("{source}")]
pub struct Error {
    pub(crate) day: usize,
    #[source]
    pub(crate) source: ErrorKind,
}

impl Error {
    pub(crate) fn invalid_day(day: usize) -> Self {
        Error {
            day,
            source: ErrorKind::InvalidDay(day),
        }
    }

    pub(crate) fn not_implemented(day: usize) -> Self {
        Error {
            day,
            source: ErrorKind::DayNotImplemented(day),
        }
    }

    pub(crate) fn input_error(day: usize, err: input::error::Error) -> Self {
        Error {
            day,
            source: ErrorKind::InputError(err),
        }
    }

    pub(crate) fn parse_int_error(day: usize, err: std::num::ParseIntError, num: String) -> Self {
        Error {
            day,
            source: ErrorKind::ParseIntError(err, num),
        }
    }
}
