use std::fmt;

use clap::ValueEnum;

#[derive(Debug, Clone)]
pub struct Selection {
    challenges: Vec<Challenge>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[repr(i32)]
pub enum Part {
    First = 1,
    Second = 2,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First => write!(f, "part 1"),
            Self::Second => write!(f, "part 2"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Challenge {
    day: usize,
    part: Part
}

