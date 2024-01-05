use itertools::Itertools;
use petgraph::{algo::all_simple_paths, graphmap::GraphMap, Directed, EdgeType, Undirected};

use common::prelude::*;
use utils::parse_to_grid;

type Point = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Path,
    Forest,
    SteepSlope(Direction),
}

fn longest_path<Ty: EdgeType>(
    edges: Vec<(Point, Point, usize)>,
    from: Point,
    to: Point,
) -> Option<usize> {
    let graph = GraphMap::<_, _, Ty>::from_edges(edges);
    all_simple_paths(&graph, from, to, 0, None)
        .map(|path: Vec<_>| {
            path.into_iter()
                .tuple_windows()
                .map(|edge| graph[edge])
                .sum()
        })
        .max()
}

pub fn solve(part: Part) -> Result<usize> {
    let input = include_str!("../inputs/hike.txt");
    let grid = parse_to_grid(input.lines(), |ch| match ch {
        '#' => Ok(Cell::Forest),
        '.' => Ok(Cell::Path),
        '>' => Ok(Cell::SteepSlope(Direction::East)),
        'v' => Ok(Cell::SteepSlope(Direction::South)),
        _ => bail!("Wrong character: {}", ch),
    })?;
    let (nrows, ncols) = (grid.len(), grid[0].len());
    let start = (0, 1);
    let mut edges = vec![];
    let mut stack = vec![(start, Direction::South)];
    let mut visited = HashSet::new();
    while let Some((path_start, mut direction)) = stack.pop() {
        if !visited.insert((path_start, direction)) {
            continue;
        }
        let (mut r, mut c) = direction
            .next_location(path_start, (nrows, ncols))
            .context("Wrong direction")?;
        let mut path_length = 1usize;
        loop {
            let nexts = [
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::West,
            ]
            .into_iter()
            .filter(|d| d.opposite() != direction)
            .filter_map(|d| d.next_location((r, c), (nrows, ncols)).map(|loc| (loc, d)))
            .filter(|((r, c), d)| match grid[*r][*c] {
                Cell::Path => true,
                Cell::Forest => false,
                Cell::SteepSlope(slope) => part.value(d == &slope, true),
            });
            match nexts.exactly_one() {
                Ok(item) => {
                    ((r, c), direction) = item;
                    path_length += 1;
                }
                Err(it) => {
                    edges.push((path_start, (r, c), path_length));
                    for (_, d) in it {
                        stack.push(((r, c), d));
                    }
                    break;
                }
            }
        }
    }
    edges.sort_unstable();

    edges = edges
        .into_iter()
        .coalesce(|x, y| {
            if x.0 == y.0 && x.1 == y.1 {
                Ok(if x.2 >= y.2 { x } else { y })
            } else {
                Err((x, y))
            }
        })
        .collect();
    let goal = (nrows - 1, ncols - 2);
    match part {
        Part1 => longest_path::<Directed>(edges, start, goal),
        Part2 => longest_path::<Undirected>(edges, start, goal),
    }
    .context("No path found")
}

impl Direction {
    const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn next_location(self, (x, y): Point, (nrows, ncols): Point) -> Option<Point> {
        match self {
            Self::North => x.checked_sub(1).map(|i| (i, y)),
            Self::South => (x + 1 < nrows).then_some((x + 1, y)),
            Self::West => y.checked_sub(1).map(|j| (x, j)),
            Self::East => (y + 1 < ncols).then_some((x, y + 1)),
        }
    }
}
