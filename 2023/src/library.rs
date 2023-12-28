use common::prelude::*;

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |acc, b| acc.wrapping_add(b).wrapping_mul(17)) // 256 reminder is not needed
                                                                // because its u8
}

pub fn solve() -> u32 {
    let input = include_str!("../inputs/library.txt");
    let steps = input.trim_end().split(',');
    steps.map(hash).map(u32::from).sum::<u32>()
}

pub fn solve_part2() -> Result<u32> {
    let input = include_str!("../inputs/library.txt");
    let steps = input.trim_end().split(',');
    let mut boxes: [Vec<(&str, u8)>; 256] = core::array::from_fn(|_| Vec::new());
    for step in steps {
        if let Some(label) = step.strip_suffix('-') {
            boxes[hash(label) as usize].retain(|(s, _)| s != &label);
        } else {
            let (label, focal_length) = step.split_once('=').context("Wrong command")?;
            let focal_length = focal_length.parse::<u8>()?;
            ensure!(matches!(focal_length, 1..=9));
            let lenses = &mut boxes[hash(label) as usize];
            if let Some((_, old_focal_length)) = lenses.iter_mut().find(|(s, _)| s == &label) {
                *old_focal_length = focal_length;
            } else {
                lenses.push((label, focal_length));
            }
        }
    }

    let sum = boxes
        .iter()
        .enumerate()
        .flat_map(|(idx, v)| {
            v.iter().enumerate().map(move |(i, (_, focal_length))| {
                u32::from(*focal_length) * (i as u32 + 1) * (idx as u32 + 1)
            })
        })
        .sum();
    Ok(sum)
}
