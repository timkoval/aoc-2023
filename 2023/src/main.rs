use common::Part;

pub mod brick;
pub mod card;
pub mod crucible;
pub mod cube;
pub mod dig;
pub mod energizer;
pub mod expansion;
pub mod gear;
pub mod hail;
pub mod hike;
pub mod library;
pub mod map;
pub mod mirror;
pub mod pipe;
pub mod prediction;
pub mod pulse;
pub mod race;
pub mod reflector;
pub mod scratchcards;
pub mod seed;
pub mod solve;
pub mod spring;
pub mod step;
pub mod trebuchet;
pub mod wiring;
pub mod workflow;

fn main() {
    // let trebuchet = trebuchet::Trebuchet {
    //     file_lines: trebuchet::Trebuchet::load_input(),
    // };
    // let cube = cube::Cube {
    //     file_lines: cube::Cube::load_input(),
    // };
    // let gear = gear::Gear {
    //     file_lines: gear::Gear::load_input(),
    // };
    // let scratchcards = scratchcards::Scratchcards {
    //     file_lines: scratchcards::Scratchcards::load_input(),
    // };
    // let seed = seed::Seed {
    //     file_lines: seed::Seed::load_input(),
    // };
    // let race = race::Race {
    //     file_lines: race::Race::load_input(),
    // };
    println!("Part one: {}", wiring::solve(Part::Part1).unwrap());
    println!("Part two: {}", wiring::solve(Part::Part2).unwrap());
}
