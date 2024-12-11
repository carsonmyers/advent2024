use itertools::Itertools;
use std::sync::Arc;
use std::sync::Mutex;
use winnow::prelude::*;

use crate::challenge::{Error, Result, Solver};
use crate::input::helpers::{parse_lines, InputHelper};
use crate::input::Input;

#[derive(Debug, Copy, Clone)]
struct Record(usize, i64);

#[derive(Debug, Copy, Clone)]
struct Transient {
    a: Record,
    b: Record,
    diff: i64,
    diff_abs: usize,
}

impl Transient {
    fn new(a: Record, b: Record) -> Self {
        Self {
            a,
            b,
            diff: b.1 - a.1,
            diff_abs: b.1.abs_diff(a.1) as usize,
        }
    }

    fn is_increasing(&self) -> bool {
        self.diff > 0
    }
}

#[derive(Debug, Copy, Clone)]
struct Derivative {
    a: Transient,
    b: Transient,
}

impl Derivative {
    fn new(a: Transient, b: Transient) -> Self {
        Self { a, b }
    }
}

#[derive(Debug)]
pub struct Day2 {
    input: Arc<Mutex<dyn Input>>,
}

impl Day2 {
    fn read_reports(&self) -> Result<Vec<Vec<i64>>> {
        let helper = InputHelper::new(2, self.input.clone());
        let reports = self
            .read_numbers(&mut helper.all_text()?.as_str())
            .map_err(|err| Error::LineParseError(err.to_string()))?;

        Ok(reports)
    }

    fn read_numbers<'a>(&self, input: &'a mut &'a str) -> PResult<Vec<Vec<i64>>> {
        use winnow::ascii::{digit1, space1};
        use winnow::combinator::separated;

        let parser = separated(
            1..,
            digit1.try_map(|num_str: &str| num_str.parse::<i64>()),
            space1,
        );
        parse_lines(input, parser)
    }

    fn report_is_safe(report: &Vec<i64>) -> bool {
        let acceptable_deviation = Self::transients(report)
            .all(|transient| transient.diff_abs >= 1 && transient.diff_abs <= 3);

        let same_sign =
            Self::derivatives(report).all(|d| d.a.is_increasing() == d.b.is_increasing());

        acceptable_deviation && same_sign
    }

    fn mend_report(report: Vec<i64>) -> Option<Vec<i64>> {
        if Self::report_is_safe(&report) {
            return Some(report.clone());
        }

        let mut transients = Self::transients(&report);

        let unacceptable_deviation = transients.find(|t| t.diff_abs < 1 || t.diff_abs > 3);

        if let Some(transient) = unacceptable_deviation {
            return Self::mend_report_from_transient(&report, transient);
        }

        let mut derivatives = Self::derivatives(&report);

        let wrong_sign = derivatives.find(|d| d.a.is_increasing() != d.b.is_increasing());

        if let Some(derivative) = wrong_sign {
            return Self::mend_report_from_derivative(&report, derivative);
        }

        None
    }

    fn transients(report: &Vec<i64>) -> impl Iterator<Item = Transient> + use<'_> {
        report
            .into_iter()
            .enumerate()
            .map(|(i, value)| Record(i, *value))
            .tuple_windows()
            .map(|(a, b)| Transient::new(a, b))
    }

    fn derivatives(report: &Vec<i64>) -> impl Iterator<Item = Derivative> + use<'_> {
        Self::transients(report)
            .tuple_windows()
            .map(|(a, b)| Derivative::new(a, b))
    }

    fn mend_report_from_transient(report: &Vec<i64>, transient: Transient) -> Option<Vec<i64>> {
        Self::try_report_without(report, transient.a.0)
            .or_else(|| Self::try_report_without(report, transient.b.0))
    }

    fn mend_report_from_derivative(report: &Vec<i64>, derivative: Derivative) -> Option<Vec<i64>> {
        Self::try_report_without(report, derivative.a.a.0)
            .or_else(|| Self::try_report_without(report, derivative.a.b.0))
            .or_else(|| Self::try_report_without(report, derivative.b.b.0))
    }

    fn try_report_without(report: &Vec<i64>, idx: usize) -> Option<Vec<i64>> {
        let mut mended = report.clone();
        mended.remove(idx);
        if Self::report_is_safe(&mended) {
            Some(mended)
        } else {
            None
        }
    }
}

impl Solver for Day2 {
    fn new(input: Arc<Mutex<dyn Input>>) -> Self {
        Self { input }
    }

    fn solve_part_1(&self) -> Result<String> {
        let safe_reports = self
            .read_reports()?
            .into_iter()
            .filter(Self::report_is_safe)
            .count();

        Ok(safe_reports.to_string())
    }

    fn solve_part_2(&self) -> Result<String> {
        let mended_reports = self
            .read_reports()?
            .into_iter()
            .filter_map(Self::mend_report)
            .collect_vec();

        Ok(mended_reports.len().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::helpers::test_input;

    #[test]
    fn test_solve() {
        let input = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;

        let input = test_input(input);
        let solver = Day2::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_1().unwrap(), "2");
        assert_eq!(solver.solve_part_2().unwrap(), "4");
    }

    #[test]
    fn test_special() {
        let input = r#"
            22 27 30 31 32 34 37 37
        "#;

        let input = test_input(input);
        let solver = Day2::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_2().unwrap(), "0");
    }

    #[test]
    fn test_try() {
        let input = r#"
            11 12 15 18 19 18
            18 19 21 23 24 25 29
            56 59 60 61 63 66 73
            63 60 58 56 53 50 47 43
            68 66 67 69 72 73 76
        "#;

        let expected = input.lines().filter(|l| !l.trim().is_empty()).count();

        let input = test_input(input);
        let solver = Day2::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_2().unwrap(), expected.to_string());
    }
}
