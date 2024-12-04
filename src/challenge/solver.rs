use std::fmt;

use clap::ValueEnum;
use paste::paste;

use crate::challenge::{Result, Error};
use crate::input::Input;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[repr(i32)]
pub enum Part {
    First = 1,
    Second = 2,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First => write!(f, "first"),
            Self::Second => write!(f, "second"),
        }
    }
}

pub trait Solver<'a> {
    async fn new(input: &Input) -> Self;
    async fn solve(&mut self, part: Part) -> Result<String>;
}

pub async fn solve(day: usize, part: Part, input: &mut Input) -> Result<String> {
    let mut solver = get_challenge(day, input);
    solver.solve(part).await
}

macro_rules! solver_inst {
    ($day:tt, $input:ident) => {{
        Ok(Box::new(paste!{ [<day $day>]::[<Day $day>] }::new($day, $input)) as Box<dyn Solver<'a>>)
    }}
}

fn get_challenge<'a>(day: usize, input: &'a Input) -> Result<Box<dyn Solver<'a>>> {
    match day {
        1 => solver_inst!(1, input),
        day if day > 25 => Err(Error::InvalidDay(day)),
        day => Err(Error::DayNotImplemented(day)),
    }
}
