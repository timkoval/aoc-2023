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
    let mut grid = parse_to_grid(input.lines(), |ch| match ch {
        '\\' => Ok((0, Some(Object::MirrorNwSe))),
        '/' => Ok((0, Some(Object::MirrorSwNe))),
        '|' => Ok((0, Some(Object::SplitterNS))),
        '-' => Ok((0, Some(Object::SplitterEW))),
        '.' => Ok((0, None)),
        _ => bail!("Invalid character: {}", ch),
    })?;
    Ok(count_energized_from(&mut grid, (0, 0), Direction::East))
}

pub fn solve_part2() -> Result<usize> {
    let input = include_str!("../inputs/energizer.txt");
    let mut grid = parse_to_grid(input.lines(), |ch| match ch {
        '\\' => Ok((0, Some(Object::MirrorNwSe))),
        '/' => Ok((0, Some(Object::MirrorSwNe))),
        '|' => Ok((0, Some(Object::SplitterNS))),
        '-' => Ok((0, Some(Object::SplitterEW))),
        '.' => Ok((0, None)),
        _ => bail!("Invalid character: {}", ch),
    })?;

    let (nrows, ncols) = (grid.len(), grid[0].len());
    ensure!(nrows > 0 && ncols > 0);
    let starts = itertools::chain!(
        (0..nrows).map(|r| ((r, 0), Direction::East)),
        (0..nrows).map(|r| ((r, ncols - 1), Direction::West)),
        (0..ncols).map(|c| ((0, c), Direction::South)),
        (0..ncols).map(|c| ((nrows - 1, c), Direction::North)),
    );
    Ok(starts
        .map(|(location, direction)| count_energized_from(&mut grid, location, direction))
        .max()
        .expect("Empty grid"))
}

fn count_energized_from(grid: &mut Grid, location: Pt, direction: Direction) -> usize {
    let shape = (grid.len(), grid[0].len());
    let mut stack = vec![(location, direction)];
    while let Some((location, direction)) = stack.pop() {
        let (flags, ref object) = &mut grid[location.0][location.1];
        if *flags & direction.as_flag() != 0 {
            continue;
        }
        *flags |= direction.as_flag();
        match object {
            None => stack.extend(
                direction
                    .next_location(location, shape)
                    .map(|rc| (rc, direction)),
            ),
            Some(object) => stack.extend(
                object
                    .next_location(location, direction, shape)
                    .into_iter()
                    .flatten(),
            ),
        }
    }
    grid.iter_mut().flatten().fold(0, |count, (flags, _)| {
        if *flags == 0 {
            count
        } else {
            *flags = 0;
            count + 1
        }
    })
}

impl Direction {
    const fn as_flag(self) -> u8 {
        match self {
            Self::North => 1 << 0,
            Self::South => 1 << 1,
            Self::East => 1 << 2,
            Self::West => 1 << 3,
        }
    }

    fn next_location(self, (r, c): Pt, shape: Pt) -> Option<Pt> {
        Some(match self {
            Self::North => (r.checked_sub(1)?, c),
            Self::South => ((r + 1 < shape.0).then_some(r + 1)?, c),
            Self::East => (r, (c + 1 < shape.1).then_some(c + 1)?),
            Self::West => (r, c.checked_sub(1)?),
        })
    }
}

impl Object {
    fn next_location(
        self,
        location: Pt,
        direction: Direction,
        shape: Pt,
    ) -> [Option<(Pt, Direction)>; 2] {
        match (self, direction) {
            (Self::MirrorNwSe, Direction::West) | (Self::MirrorSwNe, Direction::East) => {
                [Some(Direction::North), None]
            }
            (Self::MirrorNwSe, Direction::East) | (Self::MirrorSwNe, Direction::West) => {
                [Some(Direction::South), None]
            }
            (Self::MirrorNwSe, Direction::North) | (Self::MirrorSwNe, Direction::South) => {
                [Some(Direction::West), None]
            }
            (Self::MirrorNwSe, Direction::South) | (Self::MirrorSwNe, Direction::North) => {
                [Some(Direction::East), None]
            }
            (Self::SplitterEW, Direction::East | Direction::West)
            | (Self::SplitterNS, Direction::North | Direction::South) => [Some(direction), None],
            (Self::SplitterEW, Direction::North | Direction::South) => {
                [Some(Direction::East), Some(Direction::West)]
            }
            (Self::SplitterNS, Direction::East | Direction::West) => {
                [Some(Direction::North), Some(Direction::South)]
            }
        }
        .map(|d| d.and_then(|dir| dir.next_location(location, shape).map(|rc| (rc, dir))))
    }
}
