use clap::ValueEnum;
use itertools::Itertools;
use std::fmt;
use std::str::FromStr;
use std::vec::IntoIter;
use winnow::prelude::*;

use crate::error::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[repr(i32)]
pub enum ChallengePart {
    First = 1,
    Second = 2,
}

impl ChallengePart {
    fn parse(input: &mut &str) -> PResult<Self> {
        use winnow::ascii::dec_uint;

        dec_uint
            .try_map(<Self as TryFrom<usize>>::try_from)
            .parse_next(input)
    }
}

impl TryFrom<usize> for ChallengePart {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::First),
            2 => Ok(Self::Second),
            _ => Err(Error::InvalidChallengePartError(value)),
        }
    }
}

impl fmt::Display for ChallengePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First => write!(f, "Part 1"),
            Self::Second => write!(f, "Part 2"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Challenge {
    pub day: usize,
    pub part: ChallengePart,
}

impl Challenge {
    pub fn new(day: usize, part: ChallengePart) -> Self {
        Self { day, part }
    }

    pub fn parse(input: &mut &str) -> PResult<Self> {
        use winnow::ascii::dec_uint;
        use winnow::combinator::separated_pair;

        separated_pair(dec_uint, 'p', ChallengePart::parse)
            .map(|(day, part)| Self::new(day, part))
            .parse_next(input)
    }

    pub fn from_selectors(selectors: Vec<ChallengeSelector>) -> Vec<Challenge> {
        selectors
            .into_iter()
            .flat_map(<ChallengeSelector as Into<Vec<Challenge>>>::into)
            .collect()
    }
}

impl From<ChallengeSelector> for Vec<Challenge> {
    fn from(value: ChallengeSelector) -> Self {
        use ChallengePart::*;

        match value.part {
            None => vec![
                Challenge::new(value.day, First),
                Challenge::new(value.day, Second),
            ],
            Some(part) => vec![Challenge::new(value.day, part)],
        }
    }
}

impl fmt::Display for Challenge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Day {} {}", self.day, self.part)
    }
}

impl fmt::Debug for Challenge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}p{}", self.day, self.part as i32)
    }
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct ChallengeSelector {
    pub day: usize,
    pub part: Option<ChallengePart>,
}

impl ChallengeSelector {
    pub fn new(day: usize) -> Self {
        Self { day, part: None }
    }

    pub fn new_with_part(day: usize, part: ChallengePart) -> Self {
        Self {
            day,
            part: Some(part),
        }
    }

    pub fn parse(input: &mut &str) -> PResult<Self> {
        use winnow::ascii::dec_uint;
        use winnow::combinator::alt;

        alt((
            Challenge::parse.map(<Self as From<Challenge>>::from),
            dec_uint.map(<Self as From<usize>>::from),
        ))
        .parse_next(input)
    }

    pub fn parse_list(input: &mut &str) -> PResult<Vec<Self>> {
        use winnow::combinator::{alt, separated};

        let elem = alt((
            Self::parse_range,
            Self::parse.map(|selected| vec![selected]),
        ));

        separated(1.., elem, ',')
            .map(|groups: Vec<Vec<Self>>| groups.into_iter().flatten().collect())
            .parse_next(input)
    }

    fn parse_range(input: &mut &str) -> PResult<Vec<Self>> {
        use winnow::combinator::separated_pair;

        separated_pair(Self::parse, '-', Self::parse)
            .map(|(start, end)| {
                if start == end {
                    vec![start]
                } else if start.day == end.day {
                    vec![start, end]
                } else {
                    [
                        Self::expand_range_start(start),
                        Self::expand_range_middle(start, end),
                        Self::expand_range_end(end),
                    ]
                    .into_iter()
                    .flatten()
                    .collect()
                }
            })
            .parse_next(input)
    }

    fn expand_range_start(start: ChallengeSelector) -> Vec<ChallengeSelector> {
        use ChallengePart::*;

        let mut expanded = vec![start];
        if matches!(start.part, Some(First)) {
            expanded.push(Self::new_with_part(start.day, Second))
        }

        expanded
    }

    fn expand_range_end(end: ChallengeSelector) -> Vec<ChallengeSelector> {
        use ChallengePart::*;

        let mut expanded = Vec::new();
        if matches!(end.part, Some(Second)) {
            expanded.push(Self::new_with_part(end.day, First))
        }
        expanded.push(end);

        expanded
    }

    fn expand_range_middle(
        start: ChallengeSelector,
        end: ChallengeSelector,
    ) -> Vec<ChallengeSelector> {
        ((start.day + 1)..end.day).map(Self::new).collect()
    }
}

impl From<Challenge> for ChallengeSelector {
    fn from(value: Challenge) -> Self {
        Self {
            day: value.day,
            part: Some(value.part),
        }
    }
}

impl From<usize> for ChallengeSelector {
    fn from(value: usize) -> Self {
        Self {
            day: value,
            part: None,
        }
    }
}

impl FromStr for ChallengeSelector {
    type Err = Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        Self::parse(&mut s).map_err(|_| Error::InvalidSelectionError(s.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct MultiChallengeSelector {
    selection: Vec<Challenge>,
}

impl IntoIterator for MultiChallengeSelector {
    type Item = Challenge;
    type IntoIter = IntoIter<Challenge>;

    fn into_iter(self) -> Self::IntoIter {
        self.selection.into_iter()
    }
}

impl FromStr for MultiChallengeSelector {
    type Err = Error;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        ChallengeSelector::parse_list(&mut s)
            .map(|selectors| {
                let selection = selectors
                    .into_iter()
                    .flat_map(<Vec<Challenge> as From<ChallengeSelector>>::from)
                    .collect_vec();

                Self { selection }
            })
            .map_err(|_| Error::InvalidSelectionError(s.to_string()))
    }
}
