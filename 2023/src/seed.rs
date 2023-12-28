use crate::solve::Solve;
use derivative::Derivative;
use field_accessor::FieldAccessor;
use std::collections::{HashSet, VecDeque};
use std::{collections::HashMap, print, println};

#[derive(Derivative)]
#[derivative(Debug, Default)]
struct MapEntry {
    pub destination_start: u64,
    pub source_start: u64,
    pub range: u64,
}

#[derive(Derivative, FieldAccessor)]
#[derivative(Debug, Default)]
struct SeedEntry {
    pub seed: u64,
    pub soil: u64,
    pub fertilizer: u64,
    pub water: u64,
    pub light: u64,
    pub temperature: u64,
    pub humidity: u64,
    pub location: u64,
}

impl SeedEntry {
    pub fn new(values_map: HashMap<&str, u64>) -> Self {
        let mut seed_entry = SeedEntry::default();
        for (key, value) in values_map.iter() {
            seed_entry.set(&(key.to_string()), *value);
        }
        seed_entry
    }
}

pub struct Seed {
    pub file_lines: Vec<String>,
}

impl Solve<u64> for Seed {
    fn solve(&self) -> u64 {
        let seed_ids = construct_seed_ids(self.file_lines[0].to_string());
        let maps = construct_maps(self.file_lines.clone());
        let keys = Vec::from([
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ]);

        let mut seeds: Vec<SeedEntry> = Vec::new();
        for seed_id in seed_ids.iter() {
            let mut seed_attr_value = *seed_id;
            let mut seed_attrs = HashMap::from([("seed", seed_attr_value)]);
            for key in keys.iter() {
                for map in maps.get(key).unwrap().iter() {
                    if seed_attr_value >= map.source_start
                        && seed_attr_value < map.source_start + map.range
                    {
                        let threshold = seed_attr_value - map.source_start;
                        seed_attr_value = map.destination_start + threshold;
                        break;
                    }
                }
                seed_attrs.insert(key, seed_attr_value);
            }
            let seed_entry = SeedEntry::new(seed_attrs);
            seeds.push(seed_entry);
        }

        seeds.iter().map(|s| s.location).min().unwrap()
    }

    fn solve_part_two(&self) -> u64 {
        let (starts, ranges) = construct_seed_ids2(self.file_lines[0].to_string());
        let maps = construct_maps(self.file_lines.clone());
        let keys = Vec::from([
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ]);

        let mut min_location = std::u64::MAX;
        for (start, range) in starts.iter().zip(ranges.iter()) {
            for i in 0..*range {
                let mut seed_attr_value = *start + i;
                for key in keys.iter() {
                    for map in maps.get(key).unwrap().iter() {
                        if seed_attr_value >= map.source_start
                            && seed_attr_value < map.source_start + map.range
                        {
                            let threshold = seed_attr_value - map.source_start;
                            seed_attr_value = map.destination_start + threshold;
                            break;
                        }
                    }
                }
                if seed_attr_value < min_location {
                    println!("New min location: {}", min_location);
                    min_location = seed_attr_value;
                }
            }
        }
        min_location
    }

    fn load_input() -> Vec<String> {
        let file_path = "inputs/seed.txt";
        let file = std::fs::read_to_string(file_path).expect("Error reading file");
        let file_lines = file.split("\n").map(|s| s.to_string()).collect();
        file_lines
    }
}

fn construct_seed_ids(line: String) -> Vec<u64> {
    let seeds_vec: Vec<String> = line
        .split(":")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let [_, seeds_substr] = <[String; 2]>::try_from(seeds_vec).ok().unwrap();
    let seeds: Vec<u64> = seeds_substr
        .trim()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    seeds
}

fn construct_seed_ids2(line: String) -> (Vec<u64>, Vec<u64>) {
    let nums = construct_seed_ids(line);
    let ranges: Vec<u64> = nums.iter().skip(1).step_by(2).copied().collect();
    let starts: Vec<u64> = nums.iter().step_by(2).copied().collect();
    (ranges, starts)
}

fn construct_maps<'a>(lines: Vec<String>) -> HashMap<&'a str, Vec<MapEntry>> {
    let mut maps: HashMap<&str, Vec<MapEntry>> = HashMap::new();
    let keys = Vec::from([
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ]);

    let mut key_idx: u64 = 0;
    for line in lines[3..].iter() {
        if line == "\n" || line == "" {
            continue;
        }
        if line.contains(":") {
            key_idx += 1;
            continue;
        }
        let line_vec: Vec<u64> = line
            .trim()
            .split(" ")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect();
        let [destination_start, source_start, range] = <[u64; 3]>::try_from(line_vec).ok().unwrap();

        let key = keys.get(key_idx as usize).unwrap();
        if !maps.contains_key(key) {
            maps.insert(key, Vec::new());
        }

        let key_vector = maps.get_mut(key).unwrap();
        key_vector.push(MapEntry {
            destination_start,
            source_start,
            range,
        });
    }
    maps
}
