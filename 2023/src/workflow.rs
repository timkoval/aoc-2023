use itertools::{Either, Itertools};

use common::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Operation {
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone)]
struct Condition {
    xmas_idx: u8,
    operation: Operation,
    value: u32,
}

#[derive(Debug)]
struct Item<T> {
    condition: Option<Condition>,
    result: Either<T, bool>,
}

type NamedWorkflows<'a> = Vec<(&'a str, Vec<Item<&'a str>>)>;
type IndexedWorkflows = Vec<Vec<Item<usize>>>;

pub fn solve(part: Part) -> Result<u64> {
    let input = include_str!("../inputs/workflow.txt");
    let (workflows, ratings) = input.split_once("\n\n").context("invalid input")?;
    let workflows: NamedWorkflows = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line
                .strip_suffix('}')
                .context("no }")?
                .split_once('{')
                .context("no {")?;
            rest.split(',')
                .map(Item::try_from)
                .try_collect()
                .map(|items| (name, items))
        })
        .try_collect()?;
    let names = workflows.iter().map(|(name, _)| *name).collect_vec();
    let workflows: IndexedWorkflows = workflows
        .iter()
        .map(|(_, items)| {
            items
                .iter()
                .map(|item| item.indexes_from(&names))
                .try_collect()
        })
        .try_collect()?;
    let start_idx = names.into_iter().position(|s| s == "in").context("no in")?;
    Ok(match part {
        Part1 => ratings
            .lines()
            .map(|line| {
                ensure!(line.starts_with('{') && line.ends_with('}'));
                let xmas = line[1..line.len() - 1]
                    .split(',')
                    .collect_tuple::<(_, _, _, _)>()
                    .context("invalid xmas")?;
                Ok([
                    xmas.0.strip_prefix("x=").context("invalid x")?.parse()?,
                    xmas.1.strip_prefix("m=").context("invalid m")?.parse()?,
                    xmas.2.strip_prefix("a=").context("invalid a")?.parse()?,
                    xmas.3.strip_prefix("s=").context("invalid s")?.parse()?,
                ])
            })
            .process_results(|it| {
                it.filter(|xmas: &[u32; 4]| workflows.run_process(start_idx, *xmas))
                    .map(|xmas| xmas[0] + xmas[1] + xmas[2] + xmas[3])
                    .map(u64::from)
                    .sum()
            })?,
        Part2 => {
            let valid_ranges = workflows.run_process(start_idx, [(1, 4000); 4]);
            valid_ranges
                .iter()
                .map(|part| {
                    part.iter()
                        .map(|(start, end)| end - start + 1)
                        .map(u64::from)
                        .product::<u64>()
                })
                .sum()
        }
    })
}

trait WorkflowProcess<T> {
    type Output;
    fn run_process(&self, start: usize, xmas: [T; 4]) -> Self::Output;
}

impl WorkflowProcess<u32> for IndexedWorkflows {
    type Output = bool;

    fn run_process(&self, start: usize, xmas: [u32; 4]) -> Self::Output {
        let mut idx = start;
        loop {
            for item in &self[idx] {
                let success = match item.condition {
                    None => true,
                    Some(Condition {
                        xmas_idx,
                        operation,
                        value,
                    }) => operation.eval(xmas[xmas_idx as usize], value),
                };
                if success {
                    match item.result {
                        Either::Left(i) => {
                            idx = i;
                            break;
                        }
                        Either::Right(accept) => return accept,
                    }
                }
            }
        }
    }
}

impl WorkflowProcess<(u32, u32)> for IndexedWorkflows {
    type Output = Vec<[(u32, u32); 4]>;

    fn run_process(&self, start: usize, xmas_ranges: [(u32, u32); 4]) -> Self::Output {
        let mut jobs = vec![(start, xmas_ranges)];
        let mut valid_ranges = vec![];

        while let Some((idx, mut ranges)) = jobs.pop() {
            for item in &self[idx] {
                let [success, failure] =
                    match item.condition {
                        None => [Some(ranges), None],
                        Some(Condition {
                            xmas_idx,
                            operation,
                            value,
                        }) => operation.split_range(ranges[xmas_idx as usize], value).map(
                            |opt_range| {
                                opt_range.map(|range| {
                                    let mut new = ranges;
                                    new[xmas_idx as usize] = range;
                                    new
                                })
                            },
                        ),
                    };
                if let Some(part) = success {
                    match item.result {
                        Either::Left(idx) => jobs.push((idx, part)),
                        Either::Right(false) => {}
                        Either::Right(true) => valid_ranges.push(part),
                    }
                }
                match failure {
                    Some(part) => ranges = part,
                    None => break,
                }
            }
        }
        valid_ranges
    }
}

impl Operation {
    #[inline]
    const fn eval(self, n: u32, value: u32) -> bool {
        match self {
            Self::GreaterThan => n > value,
            Self::LessThan => n < value,
        }
    }

    const fn split_range(self, range: (u32, u32), value: u32) -> [Option<(u32, u32)>; 2] {
        match self {
            Self::GreaterThan => {
                if range.1 <= value {
                    [None, Some(range)]
                } else if value < range.0 {
                    [Some(range), None]
                } else {
                    [Some((value + 1, range.1)), Some((range.0, value))]
                }
            }
            Self::LessThan => {
                if value <= range.0 {
                    [None, Some(range)]
                } else if range.1 < value {
                    [Some(range), None]
                } else {
                    [Some((range.0, value - 1)), Some((value, range.1))]
                }
            }
        }
    }
}

impl Item<&str> {
    fn indexes_from(&self, names: &[&str]) -> Result<Item<usize>> {
        let Self { condition, result } = self;
        Ok(Item {
            condition: condition.clone(),
            result: match result {
                Either::Left(name) => {
                    let idx = names.iter().position(|s| s == name);
                    Either::Left(idx.context("invalid name")?)
                }
                Either::Right(accept) => Either::Right(*accept),
            },
        })
    }
}

impl std::str::FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            ">" => Self::GreaterThan,
            "<" => Self::LessThan,
            _ => bail!("invalid operation"),
        })
    }
}

impl std::str::FromStr for Condition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            xmas_idx: match &s[..1] {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => bail!("invalid xmas index"),
            },
            operation: s[1..2].parse()?,
            value: s[2..].parse()?,
        })
    }
}

impl<'a> TryFrom<&'a str> for Item<&'a str> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let (condition, result) = match s.split_once(':') {
            Some((condition, result)) => (Some(condition.parse()?), result),
            None => (None, s),
        };
        Ok(Self {
            condition,
            result: {
                if result == "A" {
                    Either::Right(true)
                } else if result == "R" {
                    Either::Right(false)
                } else {
                    Either::Left(result)
                }
            },
        })
    }
}
