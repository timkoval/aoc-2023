use common::prelude::*;
use utils::parse_to_grid;

#[derive(Debug, Clone, Copy)]
enum Object {
    SplitterEW,
    SplitterNS,
    MirrorNwSe,
    MirrorSwNe,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Pt = (usize, usize);
type Grid = Vec<Vec<(u8, Option<Object>)>>;

pub fn solve() -> Result<usize> {
    let input = include_str!("../inputs/energizer.txt");
    let grid = parse_to_grid(input.lines(), |ch| match ch {
        '\\' => Ok((0, Some(Object::MirrorNwSe))),
        '/' => Ok((0, Some(Object::MirrorSwNe))),
        '|' => (1, Some(Object::SplitterNS)),
        '/' => (1, Some(Object::MirrorNwSe)),
        '\\' => (1, Some(Object::MirrorSwNe)),
        _ => panic!("Unknown char {}", c),
    });
}
