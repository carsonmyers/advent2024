use std::sync::{Arc, Mutex};

use winnow::prelude::*;

use crate::challenge::{Error, Result, Solver};
use crate::input::helpers::InputHelper;
use crate::input::Input;

#[derive(Debug)]
pub struct Day3 {
    input: Arc<Mutex<dyn Input>>,
}

#[derive(Debug)]
enum Cmd {
    Do(bool),
    Mul(i64, i64),
}

impl Day3 {
    fn read_muls(&self) -> Result<Vec<i64>> {
        let helper = InputHelper::new(3, self.input.clone());
        let commands = self
            .read_mul_calls(&mut helper.all_text()?.as_str())
            .map_err(|err| Error::LineParseError(err.to_string()))?;

        Ok(commands)
    }

    fn read_mul_calls<'a>(&self, input: &'a mut &'a str) -> PResult<Vec<i64>> {
        use winnow::ascii::dec_int;
        use winnow::combinator::{alt, delimited, preceded, repeat, separated_pair};
        use winnow::token::take_until;

        let mut parser = repeat(
            0..,
            preceded(
                take_until(0.., "mul"),
                alt((
                    delimited("mul(", separated_pair(dec_int, ',', dec_int), ')')
                        .map(|(a, b): (i64, i64)| a * b),
                    "mul".map(|_| 0),
                )),
            ),
        );

        parser.parse_next(input)
    }

    fn read_commands(&self) -> Result<Vec<Cmd>> {
        let helper = InputHelper::new(3, self.input.clone());
        let commands = Self::parse_commands(&mut helper.all_text()?.as_str())
            .map_err(|err| Error::LineParseError(err.to_string()))?;

        Ok(commands)
    }

    fn parse_commands(input: &mut &str) -> PResult<Vec<Cmd>> {
        use winnow::combinator::not;
        use winnow::combinator::{opt, preceded, repeat};

        let mut parser = repeat(
            0..,
            preceded(opt(not(Self::parse_command)), Self::parse_command),
        );

        parser.parse_next(input)
    }

    fn parse_command(input: &mut &str) -> PResult<Cmd> {
        use winnow::ascii::dec_int;
        use winnow::combinator::{alt, delimited, separated_pair};

        let mut parser = alt((
            "do".map(|_| Cmd::Do(true)),
            "don't()".map(|_| Cmd::Do(false)),
            delimited("mul(", separated_pair(dec_int, ',', dec_int), ")")
                .map(|(a, b)| Cmd::Mul(a, b)),
        ));

        parser.parse_next(input)
    }
}

impl Solver for Day3 {
    fn new(input: Arc<Mutex<dyn Input>>) -> Self
    where
        Self: Sized,
    {
        Self { input }
    }

    fn solve_part_1(&self) -> Result<String> {
        let result = self.read_muls()?.into_iter().sum::<i64>();

        Ok(result.to_string())
    }

    fn solve_part_2(&self) -> Result<String> {
        let commands = self.read_commands()?;

        let mut do_flag = true;
        let mut total = 0;
        for cmd in &commands {
            println!("cmd: {:?}", cmd);
            match cmd {
                Cmd::Do(flag) => {
                    println!("\tdo: {}", *flag);
                    do_flag = *flag;
                },
                Cmd::Mul(lhs, rhs) if do_flag => {
                    total += lhs * rhs;
                    println!("\tmul: {} {} {}", lhs, rhs, total);
                },
                _ => continue,
            }
        }

        Ok(total.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::helpers::test_input;

    #[test]
    fn test_solve_part_1() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

        let input = test_input(input);
        dbg!(&input);
        let solver = Day3::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_2().unwrap(), "48");
    }

    #[test]
    fn test_solve_part_2() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

        let input = test_input(input);
        let solver = Day3::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_2().unwrap(), "48");
    }

    #[test]
    fn test_simple() {
        let input = test_input(r#"mul(1,2)mul(2,2)"#);
        dbg!(&input);
        let solver = Day3::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_1().unwrap(), "6");

        let input = test_input(r#"mul(1,2)don't()mul(2,3)do()mul(3,4)"#);
        dbg!(&input);
        let solver = Day3::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_2().unwrap(), "14");
    }

    fn get_mul(input: &mut &str) -> PResult<(i64, i64)> {
        use winnow::ascii::dec_int;
        use winnow::combinator::{delimited, preceded, separated_pair};
        use winnow::error::StrContext;
        use winnow::token::take_until;

        preceded(
            take_until(0.., "mul(").context(StrContext::Label("until")),
            delimited(
                "mul(",
                separated_pair(dec_int, ',', dec_int).context(StrContext::Label("mul")),
                ')'.context(StrContext::Label("close")),
            ),
        )
        .context(StrContext::Label("delimited"))
        .parse_next(input)
    }

    fn mul(input: &mut &str) -> PResult<i64> {
        get_mul.map(|(a, b)| a * b).parse_next(input)
    }

    #[test]
    fn test_parse() {
        let mut input = "xmul(3,4)999";

        let res = mul(&mut input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12);
    }
}
