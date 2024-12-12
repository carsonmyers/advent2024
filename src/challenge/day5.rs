use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use itertools::Itertools;
use winnow::prelude::*;

use crate::challenge::Solver;
use crate::challenge::{Error, Result};
use crate::input::helpers::InputHelper;
use crate::input::Input;

#[derive(Debug)]
pub struct Day5 {
    input: Arc<Mutex<dyn Input>>,
}

type Rules = HashMap<usize, HashSet<usize>>;
type Updates = Vec<Vec<usize>>;

impl Day5 {
    fn validate_update(update: &[usize], rules: &Rules) -> bool {
        let mut seen: HashSet<usize> = HashSet::new();

        println!("update: {:?}", update);
        for page in update {
            let Some(page_rules) = rules.get(page) else {
                println!("\t{}: no rule", page);
                seen.insert(*page);
                continue;
            };

            for cannot_precede in page_rules {
                if seen.contains(cannot_precede) {
                    println!("\t{}: page {} already seen", page, cannot_precede);
                    return false;
                } else {
                    println!("\t{}: page {} has not been seen", page, cannot_precede);
                }
            }

            println!("\t{}: rule {:?} passed", page, page_rules);
            seen.insert(*page);
        }

        println!("\tPASS");
        true
    }

    fn sort_update(update: &[usize], rules: &Rules) -> Vec<usize> {
        update.iter().sorted_by(|a, b| {
            if let Some(pages) = rules.get(a) {
                if pages.contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            } else if let Some(pages) = rules.get(b) {
                if pages.contains(a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            } else {
                Ordering::Equal
            }
        }).cloned().collect_vec()
    }

    fn read_rules_and_updates(&self) -> Result<(Rules, Updates)> {
        let helper = InputHelper::new(5, self.input.clone());
        let data = self
            .parse_rules_and_updates(&mut helper.all_text()?.as_str())
            .map_err(|err| Error::LineParseError(err.to_string()))?;

        Ok(data)
    }

    fn parse_rules_and_updates(&self, input: &mut &str) -> PResult<(Rules, Updates)> {
        let (rules_lines, updates_lines) = self.parse_lines(input)?;

        let mut rules = HashMap::new();
        for rule in rules_lines {
            rules
                .entry(rule.0)
                .and_modify(|e: &mut HashSet<usize>| {
                    e.insert(rule.1);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(rule.1);
                    set
                });
        }
        dbg!(&rules);

        let mut updates = Vec::new();
        for update in updates_lines {
            updates.push(update);
        }
        dbg!(&updates);

        Ok((rules, updates))
    }
    fn parse_lines(&self, input: &mut &str) -> PResult<(Vec<(usize, usize)>, Vec<Vec<usize>>)> {
        use winnow::ascii::{dec_uint, line_ending};
        use winnow::combinator::{separated, separated_pair, terminated};
        use winnow::error::StrContext;

        (
            terminated(
                separated(1.., separated_pair(dec_uint, "|", dec_uint), line_ending),
                (line_ending, line_ending),
            )
                .context(StrContext::Label("rules section")),
            separated::<_, Vec<usize>, _, _, _, _, _>(
                1..,
                separated::<_, usize, _, _, _, _, _>(
                    1..,
                    dec_uint::<_, usize, _>,
                    ",",
                ),
                line_ending,
            )
        )
            .parse_next(input)
    }
}

impl Solver for Day5 {
    fn new(input: Arc<Mutex<dyn Input>>) -> Self
    where
        Self: Sized,
    {
        Self { input }
    }

    fn solve_part_1(&self) -> Result<String> {
        let (rules, updates) = self.read_rules_and_updates()?;

        let result = updates
            .into_iter()
            .filter(|update| Self::validate_update(update, &rules))
            .map(|pages| pages[pages.len() / 2])
            .sum::<usize>();

        Ok(result.to_string())
    }

    fn solve_part_2(&self) -> Result<String> {
        let (rules, updates) = self.read_rules_and_updates()?;

        let result = updates
            .into_iter()
            .filter(|update| !Self::validate_update(update, &rules))
            .map(|pages| Self::sort_update(&pages, &rules))
            .map(|pages| pages[pages.len() / 2])
            .sum::<usize>();

        Ok(result.to_string())
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
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "#,
        );

        let solver = Day5::new(Arc::new(Mutex::new(input)));
        assert_eq!(solver.solve_part_1().unwrap(), "143");
        assert_eq!(solver.solve_part_2().unwrap(), "123");
    }
}
