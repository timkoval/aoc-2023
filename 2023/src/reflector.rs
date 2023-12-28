use common::prelude::*;
use itertools::Itertools;
use std::str::FromStr;
use utils::parse_to_grid;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    CubeShapeRock,
    RoundedRock,
}

struct Grid {
    data: Vec<Vec<Cell>>,
    nrows: usize,
    ncols: usize,
}

pub fn solve() -> usize {
    let input = include_str!("../inputs/reflector.txt");
    let mut grid: Grid = Grid::from_str(input).unwrap();
    grid.roll_north();
    grid.total_load()
}

pub fn solve_part2() -> usize {
    let input = include_str!("../inputs/reflector.txt");
    let mut grid: Grid = Grid::from_str(input).unwrap();
    const NUM_ITERATIONS: u32 = 1_000_000_000;
    let mut cache = HashMap::with_capacity(150);
    for step in 0..NUM_ITERATIONS {
        grid.roll_cycle();
        if let Some(prev_step) = cache.insert(grid.id(), step) {
            let remaining_steps = NUM_ITERATIONS - step - 1;
            let period = step - prev_step;
            for _ in 0..remaining_steps % period {
                grid.roll_cycle();
            }
            break;
        }
    }
    grid.total_load()
}

impl Cell {
    #[inline]
    const fn is_round(self) -> bool {
        matches!(self, Cell::RoundedRock)
    }

    #[inline]
    const fn is_empty(self) -> bool {
        matches!(self, Cell::Empty)
    }
}

impl Grid {
    fn total_load(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|cell| cell.is_round()).count())
            .rev()
            .enumerate()
            .map(|(i, count)| count * (i + 1))
            .sum()
    }

    fn id(&self) -> [u64; 157] {
        let mut bits = [0; 157];
        for (idx, obj) in self.data.iter().flatten().enumerate() {
            if obj.is_round() {
                bits[idx / 64] |= 1 << (idx % 64);
            }
        }
        bits
    }

    #[inline]
    fn roll_cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    #[inline]
    fn roll_helper<C, R>(&mut self, coords: C, ray: impl Fn((usize, usize)) -> R)
    where
        C: Iterator<Item = (usize, usize)>,
        R: Iterator<Item = (usize, usize)>,
    {
        for (row, col) in coords {
            if self.data[row][col].is_round() {
                let empties = ray((row, col)).take_while(|&(r, c)| self.data[r][c].is_empty());
                if let Some((r, c)) = empties.last() {
                    self.data[row][col] = Cell::Empty;
                    self.data[r][c] = Cell::RoundedRock;
                }
            }
        }
    }

    fn roll_north(&mut self) {
        self.roll_helper(
            (1..self.nrows).cartesian_product(0..self.ncols),
            |(r, c)| (0..r).rev().map(move |i| (i, c)),
        );
    }

    fn roll_south(&mut self) {
        let nrows = self.nrows;
        self.roll_helper(
            (0..nrows - 1).rev().cartesian_product(0..self.ncols),
            |(r, c)| (r + 1..nrows).map(move |i| (i, c)),
        );
    }

    fn roll_west(&mut self) {
        self.roll_helper(
            (1..self.ncols)
                .cartesian_product(0..self.nrows)
                .map(|(c, r)| (r, c)),
            |(r, c)| (0..c).rev().map(move |i| (r, i)),
        );
    }

    fn roll_east(&mut self) {
        let ncols = self.ncols;
        self.roll_helper(
            (0..ncols - 1)
                .rev()
                .cartesian_product(0..self.nrows)
                .map(|(c, r)| (r, c)),
            |(r, c)| (c + 1..ncols).map(move |i| (r, i)),
        );
    }
}

impl std::str::FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = parse_to_grid(s.lines(), |ch| match ch {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::CubeShapeRock),
            'O' => Ok(Cell::RoundedRock),
            _ => bail!("Invalid character: {}", ch),
        })?;
        let nrows = data.len();
        let ncols = data[0].len();

        Ok(Self { data, nrows, ncols })
    }
}
