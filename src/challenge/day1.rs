use crate::challenge::Solver;
use crate::challenge::solver::Part;
use crate::input::Input;

pub struct Day1<'a> {
    input: &'a Input
}

impl<'a> Solver<'a> for Day1<'a> {
    async fn new(input: &'a mut Input) -> Self {
        Self { input }
    }

    async fn solve(&self, part: Part) -> crate::challenge::Result<String> {
        todo!()
    }
}