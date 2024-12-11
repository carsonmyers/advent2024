use std::sync::{Arc, Mutex};

use winnow::error::ContextError;
use winnow::prelude::*;

use crate::input::{Input, Result};

pub struct InputHelper {
    day: usize,
    input: Arc<Mutex<dyn Input>>,
}

impl InputHelper {
    pub fn new(day: usize, input: Arc<Mutex<dyn Input>>) -> Self {
        Self { day, input }
    }

    pub fn all_text(&self) -> Result<String> {
        let mut input = self.input.lock()?;
        input.get_input(self.day)
    }
}

pub fn parse_lines<'a, O, P>(input: &'a mut &'a str, mut parser: P) -> PResult<Vec<O>, ContextError>
where
    P: Parser<&'a str, O, ContextError>,
{
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|mut line| parser.parse_next(&mut line))
        .collect::<PResult<Vec<O>, ContextError>>()
}

#[cfg(test)]
use crate::input::SimpleInput;
#[cfg(test)]
use itertools::Itertools;

#[cfg(test)]
pub fn test_input<S: AsRef<str>>(input: S) -> SimpleInput {
    let text = input
        .as_ref()
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .map(|line| line.trim_start())
        .collect_vec();
    let mut text = text
        .into_iter()
        .rev()
        .skip_while(|line| line.is_empty())
        .collect_vec();
    text.reverse();
    let text = text.join("\n");

    SimpleInput::new(text)
}
