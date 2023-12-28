use crate::solve::Solve;

pub struct Race {
    pub file_lines: Vec<String>,
}

impl Solve<u64> for Race {
    fn solve(&self) -> u64 {
        let (times_vec, distances_vec) = construct_times_distances1(self.file_lines.clone());

        let mut ways_num = 1;
        for (time, distance) in times_vec.iter().zip(distances_vec.iter()) {
            let mut wins: u64 = 0;
            for i in 0..(time + 1) {
                if (i * (time - i)) > *distance {
                    wins += 1;
                }
            }
            ways_num *= wins;
        }
        ways_num
    }

    fn solve_part_two(&self) -> u64 {
        let (time, distance) = construct_times_distances2(self.file_lines.clone());
        let mut ways_num = 1;
        let mut wins: u64 = 0;
        for i in 0..(time + 1) {
            if (i * (time - i)) > distance {
                wins += 1;
            }
        }
        ways_num *= wins;
        ways_num
    }

    fn load_input() -> Vec<String> {
        let file_path = "inputs/race.txt";
        let file = std::fs::read_to_string(file_path).expect("Error reading file");
        let file_lines = file.split("\n").map(|s| s.to_string()).collect();
        file_lines
    }
}

fn construct_times_distances1(lines: Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let line_vec: Vec<String> = lines[0]
        .split(":")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let [_, times_substr] = <[String; 2]>::try_from(line_vec).ok().unwrap();
    let times_vec: Vec<u64> = times_substr
        .trim()
        .split(" ")
        .filter(|s| s != &"")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let line_vec: Vec<String> = lines[1]
        .split(":")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let [_, distances_substr] = <[String; 2]>::try_from(line_vec).ok().unwrap();
    let distances_vec: Vec<u64> = distances_substr
        .trim()
        .split(" ")
        .filter(|s| s != &"")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    (times_vec, distances_vec)
}

fn construct_times_distances2(lines: Vec<String>) -> (u64, u64) {
    let line_vec: Vec<String> = lines[0]
        .split(":")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let [_, times_substr] = <[String; 2]>::try_from(line_vec).ok().unwrap();
    let time: u64 = times_substr
        .trim()
        .split(" ")
        .filter(|s| s != &"")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let line_vec: Vec<String> = lines[1]
        .split(":")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let [_, distances_substr] = <[String; 2]>::try_from(line_vec).ok().unwrap();
    let distance: u64 = distances_substr
        .trim()
        .split(" ")
        .filter(|s| s != &"")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    (time, distance)
}
