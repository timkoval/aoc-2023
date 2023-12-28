use std::{collections::HashMap, println};

use crate::solve::Solve;
use std::collections::HashSet;

pub struct Scratchcards {
    pub file_lines: Vec<String>,
}

impl Solve<u128> for Scratchcards {
    fn solve(&self) -> u128 {
        let mut answer: u128 = 0;
        for idx in 0..self.file_lines.len() - 1 {
            let line = self.file_lines[idx].to_string();
            let (card_number, card_answer, _) = calculate_line(line);
            answer += card_answer;
        }
        answer
    }

    fn solve_part_two(&self) -> u128 {
        let mut cards_points_map: HashMap<u128, u128> = HashMap::new();
        let cards_tuples = (1..self.file_lines.len()).map(|k| (k as u128, 0));
        let mut cards_map: HashMap<u128, u128> = HashMap::from_iter(cards_tuples);
        for idx in 0..self.file_lines.len() - 1 {
            let line = self.file_lines[idx].to_string();
            let (card_number, card_points, answer_pow) = calculate_line(line);
            cards_points_map.insert(card_number, card_points);
            println!("{} {}", card_number, card_points);
            cards_map.insert(card_number, cards_map[&card_number] + 1);
            for num in 0..answer_pow {
                cards_map.insert(
                    card_number + (num as u128) + 1,
                    cards_map[&card_number] + cards_map[&(card_number + (num as u128) + 1)],
                );
            }
        }

        println!("{:?}", cards_map);

        let answer: u128 = cards_map.values().sum();
        answer
    }

    fn load_input() -> Vec<String> {
        let file_path = "inputs/scratchcards.txt";
        let file = std::fs::read_to_string(file_path).expect("Error reading file");
        let file_lines = file.split("\n").map(|s| s.to_string()).collect();
        file_lines
    }
}

fn calculate_line(line: String) -> (u128, u128, u32) {
    let line_vec = line
        .split(":")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let [card, numbers_substr] = <[String; 2]>::try_from(line_vec).ok().unwrap();
    let card_vec = card
        .split(" ")
        .map(|s| s.to_string())
        .filter(|s| s != &"".to_string())
        .collect::<Vec<String>>();
    let card_number = card_vec[1].to_string().parse::<u128>().unwrap();

    let numbers_vec = numbers_substr
        .split(" | ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let [winning_substr, sample_substr] = <[String; 2]>::try_from(numbers_vec).ok().unwrap();
    let winning_nums = winning_substr
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let sample_nums = sample_substr
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let answer_pow = sample_nums
        .iter()
        .filter(|s| s != &&"".to_string() && winning_nums.contains(s))
        .collect::<HashSet<&String>>()
        .len() as u32;
    if answer_pow == 0 {
        return (card_number, 0, answer_pow);
    }
    (card_number, 2u32.pow(answer_pow - 1) as u128, answer_pow)
}
