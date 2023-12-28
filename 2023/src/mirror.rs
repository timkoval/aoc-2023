use itertools::Itertools;

const ASH: u8 = b'.';
const ROCKS: u8 = b'#';

#[derive(Debug)]
pub struct Matrix {
    cols: Vec<usize>,
    rows: Vec<usize>,
}

impl Matrix {
    fn parse<'a>(matrix: &str) -> Matrix {
        let num_cols = matrix.lines().next().unwrap().len();
        let mut cols = vec![0; num_cols];

        let rows = matrix
            .lines()
            .enumerate()
            .map(|(row_idx, row)| {
                row.bytes()
                    .enumerate()
                    .map(|(col_idx, ch)| {
                        let is_rocks = (ch == ROCKS) as usize;
                        cols[col_idx] += is_rocks << row_idx;
                        is_rocks << col_idx
                    })
                    .sum::<usize>()
            })
            .collect_vec();

        Matrix { cols, rows }
    }
}

fn try_reflect(images: &Vec<usize>) -> usize {
    let mut reflected = false;
    let mut idx = 1;
    while !reflected && idx < images.len() {
        reflected = true;
        let mut lo = idx;
        let mut hi = idx - 1;
        while lo > 0 && hi < images.len() - 1 {
            lo -= 1;
            hi += 1;
            if images[lo] != images[hi] {
                reflected = false;
                break;
            }
        }
        if reflected {
            break;
        }
        idx += 1;
    }

    if reflected {
        idx
    } else {
        0
    }
}

fn one_bit_different(a: usize, b: usize) -> bool {
    let diff = a ^ b;
    diff & (diff - 1) == 0
}

fn try_reflect_smudged(images: &Vec<usize>) -> usize {
    let mut reflected = false;
    let mut smudged = false;
    let mut idx = 1;
    while idx < images.len() {
        reflected = true;
        smudged = false;
        let mut lo = idx;
        let mut hi = idx - 1;
        while lo > 0 && hi < images.len() - 1 {
            lo -= 1;
            hi += 1;
            if images[lo] == images[hi] {
                continue;
            } else if !smudged && one_bit_different(images[lo], images[hi]) {
                smudged = true;
            } else {
                reflected = false;
                break;
            }
        }
        if reflected && smudged {
            break;
        }
        idx += 1;
    }

    if reflected && smudged {
        idx
    } else {
        0
    }
}

pub fn solve() -> usize {
    let input = include_str!("../inputs/mirror.txt");
    input
        .split("\n\n")
        .map(|matrix_line| {
            let matrix = Matrix::parse(matrix_line);
            let vert = try_reflect(&matrix.cols);
            if vert > 0 {
                vert
            } else {
                let horiz = try_reflect(&matrix.rows);
                horiz * 100
            }
        })
        .sum::<usize>()
}

pub fn solve_part2() -> usize {
    let input = include_str!("../inputs/mirror.txt");
    input
        .split("\n\n")
        .map(|matrix_line| {
            let matrix = Matrix::parse(matrix_line);
            let vert = try_reflect_smudged(&matrix.cols);
            if vert > 0 {
                vert
            } else {
                let horiz = try_reflect_smudged(&matrix.rows);
                horiz * 100
            }
        })
        .sum::<usize>()
}
