use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use winnow::prelude::*;

use crate::challenge::{Error, Result, Solver};
use crate::input::helpers::{parse_lines, InputHelper};
use crate::input::Input;

#[derive(Debug)]
pub struct Day1 {
    input: Arc<Mutex<dyn Input>>,
}

impl Day1 {
    fn read_lists(&self) -> Result<(Vec<usize>, Vec<usize>)> {
        let helper = InputHelper::new(1, self.input.clone());
        let lists = self
            .read_numbers(&mut helper.all_text()?.as_str())
            .map_err(|err| Error::LineParseError(err.to_string()))?
            .into_iter()
            .unzip();

        Ok(lists)
    }
    fn read_numbers<'a>(&self, input: &'a mut &'a str) -> PResult<Vec<(usize, usize)>> {
        use winnow::ascii::{dec_uint, space1};
        use winnow::combinator::separated_pair;
        use winnow::error::{StrContext, StrContextValue};

        let parser = separated_pair(
            dec_uint
                .context(StrContext::Label("left"))
                .context(StrContext::Expected(StrContextValue::Description(
                    "unsigned int",
                ))),
            space1.context(StrContext::Label("spacing")),
            dec_uint
                .context(StrContext::Label("right"))
                .context(StrContext::Expected(StrContextValue::Description(
                    "unsigned int",
                ))),
        );
        parse_lines(input, parser)
    }
}

impl Solver for Day1 {
    fn new(input: Arc<Mutex<dyn Input>>) -> Self {
        Self { input }
    }

    fn solve_part_1(&self) -> Result<String> {
        let (mut left, mut right) = self.read_lists()?;

        left.sort();
        right.sort();

        let total_difference = left
            .into_iter()
            .zip(right.into_iter())
            .map(|(left, right)| left.abs_diff(right))
            .sum::<usize>();

        Ok(format!("{total_difference}"))
    }

    fn solve_part_2(&self) -> Result<String> {
        let (left, right) = self.read_lists()?;

        let mut counts = HashMap::new();
        for number in right {
            counts.entry(number).and_modify(|n| *n += 1).or_insert(1);
        }

        let similarity_score = left
            .into_iter()
            .map(|n| counts.get(&n).map(|c| n * c).unwrap_or(0))
            .sum::<usize>();

        Ok(format!("{similarity_score}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::helpers::test_input;

    #[test]
    fn test_solve() {
        let input = r#" 
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "#;

        let input = test_input(input);
        let solver = Day1::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_1().unwrap(), "11");
        assert_eq!(solver.solve_part_2().unwrap(), "31");
    }
}
