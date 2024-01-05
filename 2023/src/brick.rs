use std::debug_assert;

use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

#[derive(Debug)]
struct SandBrick([u32; 3], [u32; 3]);

impl SandBrick {
    fn new(start: [u32; 3], end: [u32; 3]) -> Result<Self> {
        start
            .iter()
            .zip(&end)
            .all(|(s, e)| s <= e)
            .then_some(Self(start, end))
            .context("invalid brick")
    }

    const fn height(&self) -> u32 {
        self.0[2]
    }

    const fn min_z_above(&self) -> u32 {
        self.1[2] + 1
    }

    fn fall_to(&mut self, z: u32) {
        debug_assert!(z <= self.0[2]);
        self.1[2] -= self.0[2] - z;
        self.0[2] = z;
    }

    const fn xy_intersect(&self, other: &Self) -> bool {
        !(self.1[0] < other.0[0]
            || other.1[0] < self.0[0]
            || self.1[1] < other.0[1]
            || other.1[1] < self.0[1])
    }

    const fn is_on_top(&self, other: &Self) -> bool {
        self.height() == other.min_z_above() && self.xy_intersect(other)
    }
}

pub fn solve(part: Part) -> Result<usize> {
    let input = include_str!("../inputs/brick.txt");
    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').context("No ~")?;
            SandBrick::new(
                start.split(',').map(str::parse).ok_collect_array()?,
                end.split(',').map(str::parse).ok_collect_array()?,
            )
        })
        .try_collect()?;

    let num_bricks = bricks.len();
    bricks.sort_by_key(SandBrick::height);
    for i in 0..num_bricks {
        let z = bricks[..i]
            .iter()
            .filter(|old| bricks[i].xy_intersect(old))
            .map(SandBrick::min_z_above)
            .max()
            .unwrap_or(0);
        bricks[i].fall_to(z);
    }
    Ok(match part {
        Part1 => {
            let mut disintegratables = vec![true; num_bricks];
            for (i, brick) in bricks.iter().enumerate() {
                if let Ok(idx) = (0..i)
                    .filter(|j| brick.is_on_top(&bricks[*j]))
                    .exactly_one()
                {
                    disintegratables[idx] = false;
                }
            }
            disintegratables.iter().filter(|&&b| b).count()
        }
        Part2 => {
            let mut below = vec![vec![]; num_bricks];
            let mut above = vec![vec![]; num_bricks];
            bricks.iter().enumerate().tuple_combinations().for_each(
                |((i0, brick0), (i1, brick1))| {
                    if brick1.is_on_top(brick0) {
                        below[i1].push(i0);
                        above[i0].push(i1);
                    }
                },
            );
            for indexes in below.iter_mut().chain(above.iter_mut()) {
                indexes.sort_unstable();
                indexes.dedup();
            }

            let mut fallables = vec![false; num_bricks];
            let mut stack = vec![];
            (0..num_bricks)
                .map(|idx| {
                    debug_assert!(fallables.iter().all(|&b| !b));
                    debug_assert!(stack.is_empty());
                    stack.push(idx);
                    while let Some(i) = stack.pop() {
                        if fallables[i] {
                            continue;
                        }
                        fallables[i] = true;
                        for &a in &above[i] {
                            if below[a].iter().all(|&b| fallables[b]) {
                                stack.push(a);
                            }
                        }
                    }
                    let count = fallables.iter_mut().filter(|&&mut b| b).fold(0, |acc, f| {
                        *f = false;
                        acc + 1
                    });
                    count - 1
                })
                .sum()
        }
    })
}
