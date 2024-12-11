use crate::challenge;
use crate::input;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid challenge part: {0}")]
    InvalidChallengePartError(usize),
    #[error("invalid challenge selection: {0}")]
    InvalidSelectionError(String),
    #[error("invalid challenge selection range: {0}")]
    InvalidSelectionRangeError(String),
    #[error("error running challenge: {0}")]
    ChallengeError(#[from] challenge::Error),
    #[error("input error: {0}")]
    InputError(#[from] input::Error),
}

pub fn thread_panic_string(err: JoinError) -> String {
    if err.is_panic() {
        let err = err.into_panic();
        if let Some(s) = err.downcast_ref::<String>() {
            return s.to_string();
        }
        if let Some(s) = err.downcast_ref::<&str>() {
            return s.to_string();
        }

        return format!("{:?}", err);
    }

    format!("{:?}", err)
}
