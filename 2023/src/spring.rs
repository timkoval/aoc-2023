use itertools::Itertools;

fn process_line(spring: &str, counts: impl Iterator<Item = usize>) -> usize {
    let counts = counts.collect_vec();

    let spring = format!(".{}", spring.trim_end_matches('.'));
    let spring = spring.chars().collect_vec();

    let mut dp = vec![0; spring.len() + 1];
    dp[0] = 1;

    for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }

    for count in counts {
        let mut n_dp = vec![0; spring.len() + 1];
        let mut chunk = 0;

        for (i, &c) in spring.iter().enumerate() {
            if c != '.' {
                chunk += 1;
            } else {
                chunk = 0;
            }

            if c != '#' {
                n_dp[i + 1] += n_dp[i];
            }

            if chunk >= count && spring[i - count] != '#' {
                n_dp[i + 1] += dp[i - count];
            }
        }

        dp = n_dp;
    }

    *dp.last().unwrap()
}

pub fn solve() -> usize {
    let input = include_str!("../inputs/spring.txt");
    input
        .lines()
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|number| number.parse::<usize>().unwrap());
            process_line(spring, counts)
        })
        .sum::<usize>()
}

pub fn solve_part2() -> usize {
    let input = include_str!("../inputs/spring.txt");
    input
        .lines()
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();

            let spring = std::iter::once(spring).cycle().take(5).join("?");

            let counts = counts
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect_vec();
            let n = counts.len();

            process_line(&spring, counts.into_iter().cycle().take(5 * n))
        })
        .sum::<usize>()
}
