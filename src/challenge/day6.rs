use std::sync::{Arc, Mutex};

use crate::challenge::{Error, Result, Solver};
use crate::input::helpers::InputHelper;
use crate::input::Input;

#[derive(Debug)]
pub struct Day6 {
    input: Arc<Mutex<dyn Input>>,
}

impl Day6 {
    fn parse_map(&self) -> Result<Map> {
        let helper = InputHelper::new(6, self.input.clone());
        Map::parse(helper.all_text()?.as_str())
    }
}

impl Solver for Day6 {
    fn new(input: Arc<Mutex<dyn Input>>) -> Self
    where
        Self: Sized,
    {
        Self { input }
    }

    fn solve_part_1(&self) -> Result<String> {
        let map = self.parse_map()?;
        todo!()
    }

    fn solve_part_2(&self) -> Result<String> {
        todo!()
    }
}

#[derive(Debug)]
struct Map {
    start: TraverseFrom,
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}

#[derive(Copy, Clone, Debug)]
struct TraverseFrom {
    from: Pos,
    direction: Direction,
}

impl TraverseFrom {
    fn new(from: Pos, direction: Direction) -> Self {
        Self { from, direction }
    }
}

#[derive(Debug)]
enum TraversedTo {
    Obstacle(Pos, Direction, usize),
    Out(usize),
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => Err(Error::InvalidDirectionError(value)),
        }
    }
}

impl Map {
    fn parse(input: &str) -> Result<Self> {
        let data = input.lines().map(|line| line.chars().collect()).collect();

        let width = data
            .get(0)
            .ok_or(Error::LineParseError("no data".into()))
            .len();
        let height = data.len();

        let mut rows = Vec::with_capacity(height);
        let mut cols = vec![vec![]; width];
        let mut start = None;

        for (y, chars) in data.iter().enumerate() {
            let mut row = vec![];
            for (x, c) in chars {
                match c {
                    '#' => {
                        row.push(x);
                        cols.get_mut(x)
                            .ok_or(Error::LineParseError("uneven input".into()))
                            .push(y);
                    },
                    '.' => (),
                    dir if start.is_none() => start = Some(dir.into()?),
                    _ => return Err(Error::LineParseError("multiple starting positions".into())),
                }
            }

            rows.push(row);
        }

        let start = start.ok_or(Error::LineParseError("no starting position".into()))?;

        Self { start, rows, cols }
    }

    fn traverse(&self, from: TraverseFrom) -> TraversedTo {
        let next = match from.1 {
            Direction::Up => self.cols[from.0 .0].iter().take(from.0 .1).rev().next(),
            Direction::Right => self.rows[from.0 .1].iter().skip(from.0 .0 + 1).next(),
            Direction::Down => self.cols[from.0 .0].iter().skip(from.0 .1 + 1).next(),
            Direction::Left => self.rows[from.0 .1].iter().take(from.0 .0).rev().next(),
        };

        todo!()
    }
}
