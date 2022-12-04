use thiserror::Error;

use crate::challenge;
use crate::input;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid day `{0}`")]
    InvalidDayError(usize),
    #[error("error running challenge: {0}")]
    ChallengeError(#[from] challenge::error::Error),
    #[error("input error: {0}")]
    InputError(#[from] input::error::Error),
}
