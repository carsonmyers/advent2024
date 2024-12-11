mod day1;
mod day2;
mod day3;
mod error;
mod solver;

pub use error::*;
pub use solver::*;

pub type Result<T> = std::result::Result<T, Error>;
