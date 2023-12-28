pub fn solve() -> i64 {
    let file = include_str!("../inputs/prediction.txt");
    file.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .fold(0, |sum, numbers| {
            sum + calculate_next(&numbers) + numbers.last().unwrap()
        })
}

pub fn solve_part2() -> i64 {
    let file = include_str!("../inputs/prediction.txt");
    file.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .fold(0, |sum, numbers| {
            sum + numbers.first().unwrap() - calculate_previous(&numbers)
        })
}

fn calculate_next(numbers: &Vec<i64>) -> i64 {
    if !numbers.into_iter().any(|&x| x != 0) {
        return 0;
    }
    let mut differences: Vec<i64> = vec![];
    for pair in numbers.windows(2) {
        differences.push(pair[1] - pair[0]);
    }
    calculate_next(&differences) + differences.last().unwrap()
}

fn calculate_previous(numbers: &Vec<i64>) -> i64 {
    if !numbers.into_iter().any(|&x| x != 0) {
        return 0;
    }
    let mut differences: Vec<i64> = vec![];
    for pair in numbers.windows(2) {
        differences.push(pair[1] - pair[0]);
    }
    differences.first().unwrap() - calculate_previous(&differences)
}
