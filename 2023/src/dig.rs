use itertools::Itertools;

use common::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn solve(part: Part) -> Result<u64> {
    let input = include_str!("../inputs/dig.txt");
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction_side = parts.next().context("empty line")?;
            let number = parts.next().context("no number")?;
            let color = parts.next().context("no color")?;
            ensure!(parts.next().is_none(), "too many parts");
            Ok(match part {
                Part1 => {
                    let direction = match direction_side {
                        "U" => Direction::North,
                        "D" => Direction::South,
                        "R" => Direction::East,
                        "L" => Direction::West,
                        _ => bail!("invalid direction"),
                    };
                    (direction, number.parse()?)
                }
                Part2 => {
                    let color = color
                        .strip_prefix("(#")
                        .context("Wrong color prefix")?
                        .strip_suffix(')')
                        .context("Wrong color suffix")?;
                    ensure!(color.len() == 6, "Wrong color length");
                    let (number, direction_side) = color.split_at(5);
                    let direction = match direction_side {
                        "0" => Direction::East,
                        "1" => Direction::South,
                        "2" => Direction::West,
                        "3" => Direction::North,
                        _ => bail!("invalid direction"),
                    };
                    (direction, i64::from_str_radix(number, 16)?)
                }
            })
        })
        .process_results(|it| {
            let mut location = (0, 0);
            let mut double_area = 0;
            let mut perimeter = 0;

            std::iter::once(location)
                .chain(it.map(|(direction, number)| {
                    match direction {
                        Direction::North => location.0 -= number,
                        Direction::South => location.0 += number,
                        Direction::East => location.1 += number,
                        Direction::West => location.1 -= number,
                    };
                    location
                }))
                .tuple_windows()
                .for_each(|((a, b), (c, d))| {
                    double_area += a * d - b * c;
                    perimeter += a.abs_diff(c).max(b.abs_diff(d));
                });
            (location == (0, 0))
                .then(|| double_area.abs_diff(0) / 2 + perimeter / 2 + 1)
                .context("The polygon is not closed")
        })?
}
