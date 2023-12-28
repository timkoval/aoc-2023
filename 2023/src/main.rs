pub mod card;
pub mod cube;
pub mod energizer;
pub mod expansion;
pub mod gear;
pub mod library;
pub mod map;
pub mod mirror;
pub mod pipe;
pub mod prediction;
pub mod race;
pub mod reflector;
pub mod scratchcards;
pub mod seed;
pub mod solve;
pub mod spring;
pub mod trebuchet;

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
    println!("Part one: {}", energizer::solve());
    println!("Part two: {}", library::solve_part2().unwrap());
}
