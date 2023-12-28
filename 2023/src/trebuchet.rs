use std::{print, println};

use crate::solve::Solve;
use std::collections::{HashMap, VecDeque};

pub struct Trebuchet {
    pub file_lines: Vec<String>,
}

impl Solve<u32> for Trebuchet {
    fn solve(&self) -> u32 {
        let mut answer = 0;
        for line in self.file_lines.iter() {
            let digits: VecDeque<Option<u32>> = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10))
                .collect();
            let mut last_digit: u32 = 0;
            let mut first_digit: u32 = 0;
            if digits.len() > 0 {
                if digits.get(digits.len() - 1).is_some() {
                    last_digit = digits.get(digits.len() - 1).unwrap().unwrap_or(0);
                }
                if digits.get(0).is_some() {
                    first_digit = digits.get(0).unwrap().unwrap_or(0);
                };
            };
            println!("{}", line);
            answer += first_digit * 10 as u32 + last_digit;
            println!("{}", first_digit * 10 as u32 + last_digit);
        }
        answer
    }
    fn solve_part_two(&self) -> u32 {
        let mut answer = 0;
        for line in self.file_lines.iter() {
            let mut first_digit: u32 = 0;
            println!("{}", line);
            for idx in 0..line.len() {
                println!("{}", line[idx..].to_string());
                let ch = line[idx..].to_string();
                let val = try_digit(ch);
                if val > 0 {
                    first_digit = val;
                    break;
                }
                continue;
            }
            let mut last_digit: u32 = 0;
            for idx in 1..line.len() + 1 {
                let ch = line[line.len() - idx..].to_string();
                let val = try_digit(ch);
                if val > 0 {
                    last_digit = val;
                    break;
                }
                continue;
            }
            answer += first_digit * 10 as u32 + last_digit;
            println!("{}", first_digit * 10 as u32 + last_digit);
        }
        answer
    }

    fn load_input() -> Vec<String> {
        let file_path = "inputs/trebuchet.txt";
        let file = std::fs::read_to_string(file_path).expect("Error reading file");
        let file_lines = file.split("\n").map(|s| s.to_string()).collect();
        file_lines
    }
}

fn try_digit(substr: String) -> u32 {
    let digits = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    if substr.chars().nth(0).unwrap_or('%').is_digit(10) {
        return substr.chars().nth(0).unwrap_or('%').to_digit(10).unwrap();
    }
    for (key, value) in digits.iter() {
        if substr.starts_with(key) {
            return value.to_digit(10).unwrap();
        }
    }
    return 0;
}
