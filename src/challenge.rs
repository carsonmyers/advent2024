mod error;
mod solver;
mod day1;

use std::fmt;

use clap::ValueEnum;
use futures::stream::{self, FuturesOrdered, StreamExt};

use crate::input::Input;
use solver::Part;

pub use error::*;
pub use solver::*;

pub type Result<T> = std::result::Result<T, Error>;

pub async fn run_all_challenges(input_svc: &Input) -> Result<Vec<Vec<String>>> {
    let results = stream::iter(1..=25)
        .map(|day| {
            run_challenge(
                day,
                vec![Part::First, Part::Second],
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
            Err(Error::DayNotImplemented(_)) => None,
            result @ Err(_) => Some(result),
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(results)
}

pub async fn run_challenge(
    day: usize,
    parts: Vec<Part>,
    input_svc: &Input,
) -> Result<Vec<String>> {
    /*
    let challenge = get_challenge(day, input_svc).await?;

    let result = parts
        .into_iter()
        .map(|part| challenge.run(part))
        .collect::<Result<Vec<_>>>()?;

    Ok(result)
     */

    todo!()
}

