use super::{Challenge, ChallengePart, Result};

#[derive(Copy, Clone, Debug)]
#[repr(i64)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    /// Get the score resulting from a round of rock-paper-scissors.
    ///
    /// Since rock = 1, paper = 2, and scissors = 3, and they win in descending order
    /// (scissors beats paper, paper beats rock, rock beats scissors), the result
    /// can be determined by subtracting the opponent move from our move. If we win
    /// with scissors or paper, the result is 3 - 2 = 1 or 2 - 1 = 1 respectively.
    /// If we throw rock, it wraps around back to scissors, so 1 - 3 = -2.
    ///
    /// The result will be 0 if we throw the same shape since the values will always be
    /// the same. Any other result is a loss for us (it's the inverse of the win condition,
    /// losing with -1 or 2)
    fn score(opponent: &Move, player: &Move) -> i64 {
        let shape_score = *player as i64;
        let result = Self::round(opponent, player);

        shape_score + result as i64
    }

    fn round(opponent: &Move, player: &Move) -> RoundResult {
        match *player as i64 - *opponent as i64 {
            1 | -2 => RoundResult::Win,
            0 => RoundResult::Draw,
            -1 | 2 => RoundResult::Lose,
            n => panic!("unexpected result {}", n),
        }
    }

    fn sign_to(opponent: &Move, result: &RoundResult) -> Move {
        match result {
            RoundResult::Win => Move::from(*opponent as i64 + 1),
            RoundResult::Draw => Move::from(*opponent),
            RoundResult::Lose => Move::from(*opponent as i64 - 1),
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            c => panic!("unsupported character in input: {}", c),
        }
    }
}

impl From<i64> for Move {
    fn from(n: i64) -> Self {
        match n {
            0 => Move::Scissors,
            1 => Move::Rock,
            2 => Move::Paper,
            3 => Move::Scissors,
            4 => Move::Rock,
            _ => panic!("unsupported number for move: {}", n),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(i64)]
enum RoundResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl From<char> for RoundResult {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            c => panic!("unsupported character in input: {}", c),
        }
    }
}

pub struct Day2<T: AsRef<str>> {
    r#in: Vec<T>,
}

impl<T: AsRef<str>> Day2<T> {
    pub fn new(r#in: Vec<T>) -> Self {
        Self { r#in }
    }

    fn run_first(&self) -> Result<i64> {
        Ok(self
            .r#in
            .iter()
            .filter_map(|line| {
                let mut round = line.as_ref().chars().filter(|c| c.is_ascii_alphabetic());

                let left = round.next();
                let right = round.next();

                match (left, right) {
                    (Some(l), Some(r)) => Some(Move::score(&l.into(), &r.into())),
                    _ => None,
                }
            })
            .sum())
    }

    fn run_second(&self) -> Result<i64> {
        Ok(self
            .r#in
            .iter()
            .filter_map(|line| {
                let mut round = line.as_ref().chars().filter(|c| c.is_ascii_alphabetic());

                let left = round.next();
                let right = round.next();

                match (left, right) {
                    (Some(l), Some(r)) => {
                        let sign = Move::sign_to(&l.into(), &r.into());
                        Some(Move::score(&l.into(), &sign))
                    }
                    _ => None,
                }
            })
            .sum())
    }
}

impl<T: AsRef<str>> Challenge<T> for Day2<T> {
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
            A Y
            B X
            C Z
        "#
        .lines()
        .skip(1)
        .map(|line| line.trim())
        .collect::<Vec<_>>();

        let challenge = Day2::new(input);

        let result = challenge.run_first();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 15);

        let result = challenge.run_second();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12);
    }
}
