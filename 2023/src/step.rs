use std::{assert_eq, collections::VecDeque, matches};

use common::prelude::*;
use utils::{neighbors, parse_to_grid_with_loc};

#[derive(Debug, Clone)]
enum Cell {
    Rock,
    GardenPlot(Option<u32>),
}

#[derive(Debug)]
struct Garden {
    start: (usize, usize),
    grid: Vec<Vec<Cell>>,
}

pub fn solve(part: Part) -> Result<u64> {
    let input = include_str!("../inputs/step.txt");
    let mut garden: Garden = input.parse()?;
    garden.read_distances();
    Ok(match part {
        Part1 => garden.exact_steps_no_infinite(64),
        Part2 => garden.exact_steps(26501365),
    })
}

impl Garden {
    fn shape(&self) -> (usize, usize) {
        let nrows = self.grid.len();
        let ncols = self.grid[0].len();
        (nrows, ncols)
    }

    fn read_distances(&mut self) {
        let (nrows, ncols) = self.shape();
        let mut queue = VecDeque::from([(0, self.start)]);
        while let Some((dist, (row, column))) = queue.pop_front() {
            if let Cell::GardenPlot(rc_dist @ None) = &mut self.grid[row][column] {
                *rc_dist = Some(dist);
                for (r0, c0) in neighbors((row, column), nrows, ncols, false) {
                    if matches!(self.grid[r0][c0], Cell::GardenPlot(None)) {
                        queue.push_back((dist + 1, (r0, c0)));
                    }
                }
            }
        }
    }

    fn exact_steps_no_infinite(&self, steps: u32) -> u64 {
        self.grid.iter().flatten().filter(|cell| matches!(cell, Cell::GardenPlot(Some(dist)) if *dist <= steps && *dist % 2 == steps % 2)).count() as u64
    }

    fn exact_steps(&self, steps: u32) -> u64 {
        let size = self.shape().0;
        let strictly_in_corner = |r, c| {
            usize::min(size - 1 - r, r) + c < (size - 1) / 2
                || usize::max(size - 1 - r, r) + c > 3 * (size - 1) / 2
        };
        let in_corner = |r, c| {
            usize::min(size - 1 - r, r) + c <= (size - 1) / 2
                || usize::max(size - 1 - r, r) + c >= 3 * (size - 1) / 2
        };
        let whole_even = self
            .grid
            .iter()
            .flatten()
            .filter(|cell| matches!(cell, Cell::GardenPlot(Some(dist)) if *dist % 2 == 0))
            .count() as u64;
        let whole_odd = self
            .grid
            .iter()
            .flatten()
            .filter(|cell| matches!(cell, Cell::GardenPlot(Some(dist)) if *dist % 2 == 1))
            .count() as u64;

        let (center, other) = if steps % 2 == 0 {
            (whole_even, whole_odd)
        } else {
            (whole_odd, whole_even)
        };
        let middle = self.start.0;
        let q = (steps - middle as u32) / size as u32;
        let r = (steps - middle as u32) % size as u32;
        assert_eq!(r, 0);
        center
            * (1 + 4
                * (2..)
                    .step_by(2)
                    .take_while(|k| *k < q)
                    .map(u64::from)
                    .sum::<u64>())
            + other
                * 4
                * (1..)
                    .step_by(2)
                    .take_while(|k| *k < q)
                    .map(u64::from)
                    .sum::<u64>()
            + self
                .grid
                .iter()
                .enumerate()
                .flat_map(|(r, col)| col.iter().enumerate().map(move |(c, cell)| ((r, c), cell)))
                .map(|((r, c), cell)| {
                    if let Cell::GardenPlot(Some(dist)) = cell {
                        if dist % 2 == q % 2 {
                            if in_corner(r, c) {
                                u64::from(q)
                            } else {
                                0
                            }
                        } else {
                            if strictly_in_corner(r, c) {
                                3 * u64::from(q - 1) + 2
                            } else {
                                4 * u64::from(q - 1) + 4
                            }
                        }
                    } else {
                        0
                    }
                })
                .sum::<u64>()
    }
}

impl std::str::FromStr for Garden {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let grid = parse_to_grid_with_loc(s.lines(), |loc, ch| match ch {
            '#' => Ok(Cell::Rock),
            '.' => Ok(Cell::GardenPlot(None)),
            'S' => {
                start = Some(loc);
                Ok(Cell::GardenPlot(None))
            }
            _ => bail!("Invalid character: {}", ch),
        })?;
        let start = start.context("No start")?;
        Ok(Self { start, grid })
    }
}
