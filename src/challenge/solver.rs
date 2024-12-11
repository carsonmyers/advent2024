use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::sync::{Arc, Mutex};

use paste::paste;
use tokio::task::{spawn_blocking, JoinSet};

use crate::challenge::*;
use crate::error::thread_panic_string;
use crate::input::Input;
use crate::select::{Challenge, ChallengePart};

pub trait Solver: Debug + Send + Sync {
    fn new(input: Arc<Mutex<dyn Input>>) -> Self
    where
        Self: Sized;

    fn solve_part_1(&self) -> Result<String>;
    fn solve_part_2(&self) -> Result<String>;

    fn solve(&self, part: ChallengePart) -> Result<String> {
        use ChallengePart::*;
        match part {
            First => self.solve_part_1(),
            Second => self.solve_part_2(),
        }
    }
}

#[derive(Debug)]
pub struct Solution {
    challenge: Challenge,
    solution: Result<String>,
}

impl Solution {
    fn error(challenge: Challenge, err: Error) -> Self {
        Self {
            challenge,
            solution: Err(err),
        }
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.challenge.eq(&other.challenge)
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.challenge.partial_cmp(&other.challenge)
    }
}

impl Eq for Solution {}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.challenge.cmp(&other.challenge)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.solution {
            Ok(solution) => write!(f, "{}: {}", self.challenge, solution),
            Err(err) => write!(f, "{}: ERROR {}", self.challenge, err),
        }
    }
}

pub async fn solve(challenge: Challenge, input: Arc<Mutex<dyn Input>>) -> Solution {
    let solver = get_challenge(challenge.day, input);
    let Ok(solver) = solver else {
        return Solution::error(challenge, solver.unwrap_err());
    };

    let solution = spawn_blocking(move || {
        let solution = solver.solve(challenge.part);
        Solution {
            challenge,
            solution,
        }
    })
        .await;

    let Ok(solution) = solution else {
        return Solution::error(
            challenge,
            Error::SolverPanicError(thread_panic_string(solution.unwrap_err())),
        );
    };

    solution
}

pub async fn solve_all(
    challenges: Vec<Challenge>,
    input: Arc<Mutex<impl Input + 'static>>,
) -> Vec<Solution> {
    let mut join_set = JoinSet::new();
    for challenge in challenges {
        join_set.spawn(solve(challenge, input.clone()));
    }

    let mut result = Vec::new();
    while let Some(solution) = join_set.join_next().await {
        let Ok(solution) = solution else {
            panic!("unhandled join error: {:?}", solution.unwrap_err())
        };

        result.push(solution);
    }

    result.sort();
    result
}

macro_rules! solver_inst {
    ($day:tt, $input:ident) => {{
        Ok(Box::new(<paste! { [<day $day>]::[<Day $day>] }>::new($input)) as Box<dyn Solver>)
    }};
}

fn get_challenge<'a>(day: usize, input: Arc<Mutex<dyn Input>>) -> Result<Box<dyn Solver>> {
    match day {
        1 => solver_inst!(1, input),
        2 => solver_inst!(2, input),
        3 => solver_inst!(3, input),
        4 => solver_inst!(4, input),
        day if day > 25 => Err(Error::InvalidDay(day)),
        day => Err(Error::DayNotImplemented(day)),
    }
}
