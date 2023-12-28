pub fn solve() -> usize {
    let input = include_bytes!("../inputs/expansion.txt");
    let size = input.iter().position(|&c| c == b'\n').unwrap();
    let (mut xx, mut yy) = (vec![0; size], vec![0; size]);

    input
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'#')
        .for_each(|(i, _)| {
            xx[i % (size + 1)] += 1;
            yy[i / (size + 1)] += 1;
        });
    dist(&xx, 1) + dist(&yy, 1)
}

pub fn solve_part2() -> usize {
    let input = include_bytes!("../inputs/expansion.txt");
    let size = input.iter().position(|&c| c == b'\n').unwrap();
    let (mut xx, mut yy) = (vec![0; size], vec![0; size]);

    input
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'#')
        .for_each(|(i, _)| {
            xx[i % (size + 1)] += 1;
            yy[i / (size + 1)] += 1;
        });
    dist(&xx, 999_999) + dist(&yy, 999_999)
}

#[inline(always)]
fn dist(counts: &[usize], inc: usize) -> usize {
    let (mut gaps, mut sum, mut items, mut dist) = (0, 0, 0, 0);
    for (idx, count) in counts.iter().enumerate() {
        if *count > 0 {
            let expanded = idx + inc * gaps;
            dist += count * (items * expanded - sum);
            sum += expanded * count;
            items += count;
        } else {
            gaps += 1;
        }
    }
    dist
}
