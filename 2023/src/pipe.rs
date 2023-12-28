use core::ops::Add;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: isize,
    pub y: isize,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct CellValue {
    char: char,
    neighbors: Vec<Point>,
}

struct Grid {
    cells: HashMap<Point, CellValue>,
    starting_cell: Point,
}

struct GridLoop<'a> {
    grid: &'a Grid,
    previous: Option<Point>,
    current: Option<Point>,
}

impl<'a> GridLoop<'a> {
    fn new(grid: &'a Grid) -> GridLoop<'a> {
        GridLoop {
            grid,
            previous: None,
            current: None,
        }
    }
}

impl Iterator for GridLoop<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let start = self.grid.starting_cell;

        if self.previous.is_none() {
            self.previous = Some(start);
            self.current = Some(self.grid.cells.get(&start).unwrap().neighbors[0]);

            return self.current;
        }

        let current = self.current.unwrap();

        if current == start {
            return None;
        }

        let previous = self.previous.unwrap();
        let value = self.grid.cells.get(&current).unwrap();

        let next = value.neighbors.iter().find(|&&p| p != previous).unwrap();

        self.previous = self.current;
        self.current = Some(*next);

        self.current
    }
}

impl Grid {
    fn new(contents: &str) -> Self {
        let mut cells = HashMap::new();
        let mut starting_cell = Point { x: 0, y: 0 };

        for (y, line) in contents.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                cells.insert(
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                    CellValue {
                        char,
                        neighbors: vec![],
                    },
                );
            }
        }

        for (cell, value) in cells.iter_mut() {
            let neighbors: Vec<Point> = match value.char {
                '|' => {
                    vec![*cell + Point { x: 0, y: -1 }, *cell + Point { x: 0, y: 1 }]
                }
                '-' => {
                    vec![*cell + Point { x: -1, y: 0 }, *cell + Point { x: 1, y: 0 }]
                }
                'L' => {
                    vec![*cell + Point { x: 0, y: -1 }, *cell + Point { x: 1, y: 0 }]
                }
                'J' => {
                    vec![*cell + Point { x: 0, y: -1 }, *cell + Point { x: -1, y: 0 }]
                }
                '7' => {
                    vec![*cell + Point { x: 0, y: 1 }, *cell + Point { x: -1, y: 0 }]
                }
                'F' => {
                    vec![*cell + Point { x: 0, y: 1 }, *cell + Point { x: 1, y: 0 }]
                }
                '.' => {
                    continue;
                }
                'S' => {
                    // using Copy, Clone traits
                    starting_cell = *cell;

                    vec![]
                }
                c => {
                    panic!("what did you do?, {}", c)
                }
            };

            value.neighbors = neighbors;
        }

        let top = starting_cell + Point { x: 0, y: -1 };
        let bottom = starting_cell + Point { x: 0, y: 1 };
        let left = starting_cell + Point { x: -1, y: 0 };
        let right = starting_cell + Point { x: 1, y: 0 };

        let starting_point_neighbors: Vec<Point> = [top, bottom, left, right]
            .iter()
            .filter_map(|&neighbor| {
                if let Some(value) = cells.get(&neighbor) {
                    if value.neighbors.iter().any(|&p| p == starting_cell) {
                        return Some(neighbor);
                    }
                }
                None
            })
            .collect();

        if let Some(value) = cells.get_mut(&starting_cell) {
            value.neighbors = starting_point_neighbors;
        }

        Self {
            cells,
            starting_cell,
        }
    }

    fn area(&self) -> isize {
        let mut points: Vec<Point> = GridLoop::new(self).collect();
        points.push(points[0]);

        let sum: isize = points
            .windows(2)
            .map(|p| p[0].x * p[1].y - p[0].y * p[1].x)
            .sum();

        sum.abs() / 2
    }
}

pub fn solve() -> usize {
    let input = include_str!("../inputs/pipe.txt");
    let grid = Grid::new(input);

    GridLoop::new(&grid).into_iter().count() / 2
}

pub fn solve_part2() -> isize {
    let input = include_str!("../inputs/pipe.txt");
    let grid = Grid::new(input);

    let boundary_size = GridLoop::new(&grid).into_iter().count() as isize;
    grid.area() - boundary_size / 2 + 1
}
