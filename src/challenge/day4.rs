use crate::challenge::Solver;
use crate::input::Input;
use std::sync::{Arc, Mutex};

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
        todo!()
    }

    fn solve_part_2(&self) -> crate::challenge::Result<String> {
        todo!()
    }
}
