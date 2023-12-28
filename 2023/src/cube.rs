use crate::solve::Solve;
use std::{collections::HashMap, println};

pub struct Cube {
    pub file_lines: Vec<String>,
}

impl Solve<u32> for Cube {
    fn solve(&self) -> u32 {
        let mut answer: u32 = 0;
        for idx in 0..self.file_lines.len() - 1 {
            let line = self.file_lines[idx].to_string();
            let game_number = check_line(line);
            answer += game_number;
        }
        answer
    }

    fn solve_part_two(&self) -> u32 {
        let mut answer: u32 = 0;
        for idx in 0..self.file_lines.len() - 1 {
            let line = self.file_lines[idx].to_string();
            let game_number = check_line2(line);
            answer += game_number;
        }
        answer
    }

    fn load_input() -> Vec<String> {
        let file_path = "inputs/cube.txt";
        let file = std::fs::read_to_string(file_path).expect("Error reading file");
        let file_lines = file.split("\n").map(|s| s.to_string()).collect();
        file_lines
    }
}

fn check_line(line: String) -> u32 {
    let max_nums = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let line_vec = line
        .split(":")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", line_vec);
    let [game, subsets_line] = <[String; 2]>::try_from(line_vec).ok().unwrap();
    let vec1 = game
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let [_, game_number] = <[String; 2]>::try_from(vec1).ok().unwrap();
    let subsets = subsets_line
        .split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    for subset in subsets {
        let colors_with_nums = subset
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for color_with_num in colors_with_nums {
            let color_with_num_vec = color_with_num
                .trim_start()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let [num, color] = <[String; 2]>::try_from(color_with_num_vec).ok().unwrap();
            let num = num.parse::<u32>().unwrap();
            let max_num = max_nums.get(&color[..]).unwrap();
            println!("{} {}", num, max_num);
            if num > *max_num {
                return 0;
            }
        }
    }
    return game_number.parse::<u32>().unwrap();
}

fn check_line2(line: String) -> u32 {
    let mut max_nums = HashMap::from([
        (String::from("red"), 0),
        (String::from("green"), 0),
        (String::from("blue"), 0),
    ]);
    let line_vec = line
        .split(":")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", line_vec);
    let [game, subsets_line] = <[String; 2]>::try_from(line_vec).ok().unwrap();
    let vec1 = game
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let [_, game_number] = <[String; 2]>::try_from(vec1).ok().unwrap();
    let subsets = subsets_line
        .split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    for subset in subsets {
        let colors_with_nums = subset
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for color_with_num in colors_with_nums {
            let color_with_num_vec = color_with_num
                .trim_start()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let [num, color] = <[String; 2]>::try_from(color_with_num_vec).ok().unwrap();
            let num = num.parse::<u32>().unwrap();
            let max_num = max_nums.get(&color[..]).unwrap();
            println!("{} {}", num, max_num);
            if num > *max_num {
                max_nums.insert(color, num);
            }
        }
    }
    let mut answer: u32 = 1;
    for (color, num) in max_nums.iter() {
        answer *= num;
    }
    return answer;
}
