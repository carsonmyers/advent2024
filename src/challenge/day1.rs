use itertools::Itertools;

use super::error::*;
use super::{Challenge, ChallengePart, Result};

pub struct Day1<T: AsRef<str>> {
    r#in: Vec<T>,
}

impl<T: AsRef<str>> Day1<T> {
    pub fn new(r#in: Vec<T>) -> Self {
        Self { r#in }
    }

    fn run_first(&self) -> Result<i64> {
        Ok(self.get_totals()?.into_iter().max().unwrap())
    }

    fn run_second(&self) -> Result<i64> {
        Ok(self
            .get_totals()?
            .into_iter()
            .sorted()
            .rev()
            .into_iter()
            .take(3)
            .sum())
    }

    fn get_totals(&self) -> Result<Vec<i64>> {
        self.r#in
            .iter()
            .map(|line| line.as_ref())
            .batching(|it| {
                let nums = it
                    .take_while(|line| line.len() > 0)
                    .map(|line| {
                        line.parse::<i64>()
                            .map_err(|err| Error::parse_int_error(1, err, line.to_string()))
                    })
                    .collect::<Vec<_>>();

                if nums.len() > 0 {
                    Some(nums)
                } else {
                    None
                }
            })
            .map(|batch| {
                batch
                    .into_iter()
                    .fold(Ok(0i64), |acc, next| match (acc, next) {
                        (Ok(acc), Ok(num)) => Ok(acc + num),
                        (err @ Err(_), _) => err,
                        (Ok(_), err @ Err(_)) => err,
                    })
            })
            .collect::<Result<Vec<_>>>()
    }
}

impl<T: AsRef<str>> Challenge<T> for Day1<T> {
    fn run(&self, part: ChallengePart) -> Result<i64> {
        match part {
            ChallengePart::First => self.run_first(),
            ChallengePart::Second => self.run_second(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "#
        .lines()
        .skip(1)
        .map(|line| line.trim())
        .collect::<Vec<_>>();

        let challenge = Day1::new(input);

        let result = challenge.run_first();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 24000);

        let result = challenge.run_second();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 45000);
    }
}
