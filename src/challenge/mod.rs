mod day1;
mod day2;
pub mod error;

use std::fmt;

use clap::ValueEnum;
use futures::stream::{self, FuturesOrdered, StreamExt};

use error::*;

use crate::input::Input;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[repr(i32)]
pub enum ChallengePart {
    First = 1,
    Second = 2,
}

impl fmt::Display for ChallengePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First => write!(f, "first"),
            Self::Second => write!(f, "second"),
        }
    }
}

pub trait Challenge<T: AsRef<str>> {
    fn run(&self, part: ChallengePart) -> Result<i64>;
}

pub async fn run_all_challenges(input_svc: &Input) -> Result<Vec<Vec<i64>>> {
    Ok(stream::iter(1..=25)
        .map(|day| {
            run_challenge(
                day,
                vec![ChallengePart::First, ChallengePart::Second],
                input_svc,
            )
        })
        .collect::<FuturesOrdered<_>>()
        .await
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|res| match res {
            result @ Ok(_) => Some(result),
            Err(Error {
                source: ErrorKind::DayNotImplemented(_),
                ..
            }) => None,
            result @ Err(_) => Some(result),
        })
        .collect::<Result<Vec<_>>>()?)
}

pub async fn run_challenge(
    day: usize,
    parts: Vec<ChallengePart>,
    input_svc: &Input,
) -> Result<Vec<i64>> {
    let challenge = get_challenge(day, input_svc).await?;

    let result = parts
        .into_iter()
        .map(|part| challenge.run(part))
        .collect::<Result<Vec<_>>>()?;

    Ok(result)
}

/*
macro_rules! challenge {
    ($day:tt, $svc:expr => $mod:ident :: $struct:ident) => {{
        let r#in = $svc
            .get_input($day)
            .await
            .map_err(|err| Error::input_error($day, err))?;
        Ok(Box::new($mod::$struct::new(r#in)) as Box<dyn Challenge<String>>)
    }};
}
*/

async fn get_challenge(day: usize, input_svc: &Input) -> Result<Box<dyn Challenge<String>>> {
    let challenge = match day {
        //day if day == 1 => challenge!(day, input_svc => day1::Day1),
        //day if day == 2 => challenge!(day, input_svc => day2::Day2),
        day if day > 25 => Err(Error::invalid_day(day)),
        day => Err(Error::not_implemented(day)),
    }?;

    Ok(challenge)
}
