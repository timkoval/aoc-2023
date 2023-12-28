use crate::solve::Solve;
use std::{collections::HashSet, println};

pub struct Gear {
    pub file_lines: Vec<String>,
}

impl Solve<u32> for Gear {
    fn solve(&self) -> u32 {
        let mut results_set: HashSet<(u32, u32)> = HashSet::new();

        for (row_idx, row) in self.file_lines.iter().enumerate() {
            for (col_idx, character) in row.chars().enumerate() {
                if character.is_digit(10) || character == '.' {
                    continue;
                }
                for current_row in [row_idx - 1, row_idx, row_idx + 1].iter() {
                    for current_col in [col_idx - 1, col_idx, col_idx + 1].iter_mut() {
                        if *current_row < 0
                            || *current_row >= self.file_lines.len()
                            || *current_col < 0
                            || *current_col >= self.file_lines[*current_row].len()
                            || !self.file_lines[*current_row]
                                .chars()
                                .nth(*current_col)
                                .unwrap()
                                .is_digit(10)
                        {
                            continue;
                        }
                        while *current_col > 0
                            && self.file_lines[*current_row]
                                .chars()
                                .nth(*current_col - 1)
                                .unwrap()
                                .is_digit(10)
                        {
                            *current_col -= 1;
                        }
                        results_set.insert((*current_row as u32, *current_col as u32));
                    }
                }
            }
        }

        let mut answer = 0;
        for (row, col) in results_set.iter() {
            let mut number = String::new();
            let mut col = *col;
            while col < self.file_lines[*row as usize].len() as u32
                && self.file_lines[*row as usize]
                    .chars()
                    .nth(col as usize)
                    .unwrap()
                    .is_digit(10)
            {
                number.push(
                    self.file_lines[*row as usize]
                        .chars()
                        .nth(col as usize)
                        .unwrap(),
                );
                col += 1;
            }
            answer += number.parse::<u32>().unwrap();
        }
        answer
    }

    fn solve_part_two(&self) -> u32 {
        let mut answer: u32 = 0;
        for (row_idx, row) in self.file_lines.iter().enumerate() {
            for (col_idx, character) in row.chars().enumerate() {
                if character == '.' {
                    continue;
                }
                let mut results_set: HashSet<(u32, u32)> = HashSet::new();

                println!("{} {}", row_idx, col_idx);
                let row_vec: Vec<usize>;
                let col_vec: Vec<usize>;
                if row_idx > 0 {
                    row_vec = [row_idx - 1, row_idx, row_idx + 1].to_vec();
                } else {
                    row_vec = [row_idx, row_idx + 1].to_vec();
                }
                if col_idx > 0 {
                    col_vec = [col_idx - 1, col_idx, col_idx + 1].to_vec();
                } else {
                    col_vec = [col_idx, col_idx + 1].to_vec();
                }
                for curr_row in row_vec.iter() {
                    for curr_col in col_vec.iter() {
                        let current_row = *curr_row;
                        let mut current_col = *curr_col;
                        if current_row < 0
                            || current_row >= self.file_lines.len()
                            || current_col < 0
                            || current_col >= self.file_lines[current_row].len()
                            || !self.file_lines[current_row]
                                .chars()
                                .nth(current_col)
                                .unwrap()
                                .is_digit(10)
                        {
                            continue;
                        }
                        while current_col > 0
                            && self.file_lines[current_row]
                                .chars()
                                .nth(current_col - 1)
                                .unwrap()
                                .is_digit(10)
                        {
                            current_col -= 1;
                        }
                        results_set.insert((current_row as u32, current_col as u32));
                    }
                }

                if results_set.len() != 2 {
                    continue;
                }

                let mut numbers: Vec<u32> = Vec::new();
                for (row, col) in results_set.iter() {
                    let mut number = String::new();
                    let mut col = *col;
                    while col < self.file_lines[*row as usize].len() as u32
                        && self.file_lines[*row as usize]
                            .chars()
                            .nth(col as usize)
                            .unwrap()
                            .is_digit(10)
                    {
                        number.push(
                            self.file_lines[*row as usize]
                                .chars()
                                .nth(col as usize)
                                .unwrap(),
                        );
                        col += 1;
                    }
                    numbers.push(number.parse::<u32>().unwrap());
                }
                answer += numbers[0] * numbers[1];
            }
        }
        answer
    }

    fn load_input() -> Vec<String> {
        let file_path = "inputs/gear.txt";
        let file = std::fs::read_to_string(file_path).expect("Error reading file");
        let file_lines = file.split("\n").map(|s| s.to_string()).collect();
        file_lines
    }
}
