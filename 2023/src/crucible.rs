use std::collections::BinaryHeap;

use common::prelude::*;
use utils::{char10, parse_to_grid, HeuristicItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn solve(part: Part) -> Result<u16> {
    let input = include_str!("../inputs/crucible.txt");
    let grid = parse_to_grid(input.lines(), char10::<u16>)?;
    let (nrows, ncols) = (grid.len(), grid[0].len());
    ensure!(nrows > 0 && ncols > 0);
    let goal = (nrows - 1, ncols - 1);

    let mut frontier = BinaryHeap::new();
    let mut cost_so_far = HashMap::with_capacity(nrows * ncols);
    let (min_moves_num, max_moves_num) = part.value((1, 3), (4, 10));
    for direction in [Direction::East, Direction::South] {
        let Some(location) = direction.next_location_by(min_moves_num, (0, 0), (nrows, ncols))
        else {
            continue;
        };
        let heat_loss: u16 = (1..=min_moves_num)
            .map(|i| {
                let (r, c) = direction
                    .next_location_by(i, (0, 0), (nrows, ncols))
                    .expect("segment inside the grid");
                grid[r][c]
            })
            .sum();
        frontier.push(HeuristicItem::rev(
            heat_loss,
            (location, direction, min_moves_num),
        ));
        cost_so_far.insert((location, direction, min_moves_num), heat_loss);
    }
    while let Some(HeuristicItem {
        item: (location, direction, count),
        ..
    }) = frontier.pop()
    {
        if location == goal {
            break;
        };
        let heat_loss = cost_so_far[&(location, direction, count)];
        for new_direction in [
            Direction::East,
            Direction::South,
            Direction::North,
            Direction::West,
        ] {
            if new_direction == direction.opposite() {
                continue;
            };
            let (moves_num, new_count, new_location) = if new_direction == direction {
                if count >= max_moves_num {
                    continue;
                };
                (
                    1,
                    count + 1,
                    new_direction.next_location_by(1, location, (nrows, ncols)),
                )
            } else {
                if count < min_moves_num {
                    continue;
                };
                (
                    min_moves_num,
                    min_moves_num,
                    new_direction.next_location_by(min_moves_num, location, (nrows, ncols)),
                )
            };
            if let Some((r0, c0)) = new_location {
                let new_heat_loss = heat_loss
                    + (1..=moves_num)
                        .map(|i| {
                            let (r, c) = new_direction
                                .next_location_by(i, location, (nrows, ncols))
                                .expect("segment inside the grid");
                            grid[r][c]
                        })
                        .sum::<u16>();
                if new_heat_loss
                    < cost_so_far
                        .get(&((r0, c0), new_direction, new_count))
                        .copied()
                        .unwrap_or(u16::MAX)
                {
                    cost_so_far.insert(((r0, c0), new_direction, new_count), new_heat_loss);
                    frontier.push(HeuristicItem::rev(
                        new_heat_loss,
                        ((r0, c0), new_direction, new_count),
                    ));
                }
            }
        }
    }
    cost_so_far
        .iter()
        .filter_map(|((loc, ..), heat_loss)| (loc == &goal).then_some(*heat_loss))
        .min()
        .context("goal not reached!")
}

impl Direction {
    const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn next_location_by(
        self,
        amount: usize,
        (r, c): (usize, usize),
        (nrows, ncols): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Self::North => r.checked_sub(amount).map(|i| (i, c)),
            Self::South => (r + amount < nrows).then_some((r + amount, c)),
            Self::West => c.checked_sub(amount).map(|j| (r, j)),
            Self::East => (c + amount < ncols).then_some((r, c + amount)),
        }
    }
}
