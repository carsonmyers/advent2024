use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

use itertools::Itertools;

use crate::challenge::Solver;
use crate::input::helpers::InputHelper;
use crate::input::Input;

#[derive(Debug)]
pub struct Day4 {
    input: Arc<Mutex<dyn Input>>,
}

impl Solver for Day4 {
    fn new(input: Arc<Mutex<dyn Input>>) -> Self
    where
        Self: Sized,
    {
        Self { input }
    }

    fn solve_part_1(&self) -> crate::challenge::Result<String> {
        let helper = InputHelper::new(4, self.input.clone());
        let search = WordSearch::new(helper.all_text()?);

        let x = search.find_all('X');
        let m = x
            .into_iter()
            .flat_map(|pos| search.next_position(pos, 'M'))
            .collect_vec();
        let a = m
            .into_iter()
            .flat_map(|pos| search.next_position(pos, 'A'))
            .collect_vec();
        let s = a
            .into_iter()
            .flat_map(|pos| search.next_position(pos, 'S'))
            .collect_vec();

        Ok(s.len().to_string())
    }

    fn solve_part_2(&self) -> crate::challenge::Result<String> {
        use Direction::*;

        let helper = InputHelper::new(4, self.input.clone());
        let search = WordSearch::new(helper.all_text()?);

        let m = search.find_all('M');
        let a = m
            .into_iter()
            .flat_map(|pos| search.next_position(pos, 'A'))
            .collect_vec();
        let s = a
            .into_iter()
            .flat_map(|pos| search.next_position(pos, 'S'))
            .filter(|mas| matches!(mas.direction, NE | NW | SE | SW))
            .collect_vec();

        let mut pos_map: HashMap<(usize, usize), Vec<Position>> = HashMap::new();
        for pos in &s {
            pos_map
                .entry((pos.x, pos.y))
                .and_modify(|e| e.push(*pos))
                .or_insert_with(|| vec![*pos]);
        }

        let xmases = s
            .iter()
            .cloned()
            .filter(|pos| {
                match pos.direction {
                    SE => vec![(SW, (-2, 0)), (NE, (0, -2))],
                    SW => vec![(SE, (2, 0)), (NW, (0, -2))],
                    NE => vec![(SE, (0, 2)), (NW, (-2, 0))],
                    NW => vec![(SW, (0, 2)), (NE, (2, 0))],
                    _ => unreachable!(),
                }
                .into_iter()
                .filter_map(|(direction, (x, y))| {
                    search
                        .add_coords(pos, x, y)
                        .map(|(x, y)| (direction, (x, y)))
                })
                .any(|(direction, (x, y))| {
                    pos_map.get(&(x, y)).is_some_and(|other_positions| {
                        other_positions
                            .iter()
                            .any(|other| other.direction == direction)
                    })
                })
            })
            .collect_vec();

        Ok((xmases.len() / 2).to_string())
    }
}

#[derive(Debug, Copy, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Any,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Any => "??",
            Self::N => "N ",
            Self::NE => "NE",
            Self::E => " E",
            Self::SE => "SE",
            Self::S => "S ",
            Self::SW => "SW",
            Self::W => " W",
            Self::NW => "NW",
        };

        write!(f, "{}", str)
    }
}

struct WordSearch {
    rows: usize,
    cols: usize,
    data: Vec<Vec<char>>,
}

impl WordSearch {
    fn new<S: AsRef<str>>(input: S) -> Self {
        let data = input
            .as_ref()
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let rows = data.len();
        let cols = data.first().map(Vec::len).expect("at least one row");

        Self { rows, cols, data }
    }

    fn at(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y)?.get(x).cloned()
    }

    fn find_all(&self, find_char: char) -> Vec<Position> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, cols)| {
                cols.iter().enumerate().filter_map(move |(x, c)| {
                    if *c == find_char {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .map(|(x, y)| Position {
                x,
                y,
                direction: Direction::Any,
            })
            .collect_vec()
    }

    fn next_position(&self, pos: Position, find_char: char) -> Vec<Position> {
        self.next_coords(&pos)
            .into_iter()
            .filter(|(_, (x, y))| self.at(*x, *y).is_some_and(|c| c == find_char))
            .map(|(direction, (x, y))| Position { x, y, direction })
            .collect_vec()
    }

    fn next_coords(&self, pos: &Position) -> Vec<(Direction, (usize, usize))> {
        vec![
            (Direction::N, self.add_coords(pos, 0, -1)),
            (Direction::NE, self.add_coords(pos, 1, -1)),
            (Direction::E, self.add_coords(pos, 1, 0)),
            (Direction::SE, self.add_coords(pos, 1, 1)),
            (Direction::S, self.add_coords(pos, 0, 1)),
            (Direction::SW, self.add_coords(pos, -1, 1)),
            (Direction::W, self.add_coords(pos, -1, 0)),
            (Direction::NW, self.add_coords(pos, -1, -1)),
        ]
        .into_iter()
        .filter_map(|(direction, coords)| coords.map(|coords| (direction, coords)))
        .filter(|(direction, _)| match pos.direction {
            Direction::Any => true,
            _ => *direction == pos.direction,
        })
        .collect_vec()
    }

    fn add_coords(&self, pos: &Position, add_x: isize, add_y: isize) -> Option<(usize, usize)> {
        let x = match pos.x as isize + add_x {
            -1 => None,
            n if n >= self.cols as isize => None,
            n => Some(n as usize),
        };

        let y = match pos.y as isize + add_y {
            -1 => None,
            n if n >= self.rows as isize => None,
            n => Some(n as usize),
        };

        x.and_then(|x| y.and_then(|y| Some((x, y))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::helpers::test_input;

    #[test]
    fn test_solve() {
        let input = test_input(
            r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "#,
        );

        let solver = Day4::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_1().unwrap(), "18");
        assert_eq!(solver.solve_part_2().unwrap(), "9");
    }
}
