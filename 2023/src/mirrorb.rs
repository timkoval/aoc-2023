const ASH: char = '.';
const ROCKS: char = '#';

#[derive(Debug)]
pub struct Matrix {
    cols: Vec<u32>,
    rows: Vec<u32>,
}

impl Matrix {
    fn parse<'a>(matrix: &str) -> Matrix {
        let num_cols = matrix.lines().next().unwrap().len();
        let mut cols = vec![0; num_cols];
        let mut rows = vec![0; 1];
        let mut row_idx = 0;
        let mut col_idx = 0;

        for ch in matrix.chars() {
            if ch == '\n' {
                rows.push(0);
                row_idx += 1;
                col_idx = 0;
                continue;
            }
            let val = (ch == ROCKS) as u32;
            unsafe {
                let c = cols.get_unchecked_mut(col_idx);
                *c <<= 1;
                *c += val;
                let r = rows.get_unchecked_mut(row_idx);
                *r <<= 1;
                *r += val;
            }

            col_idx += 1;
        }

        Matrix { cols, rows }
    }
}

fn try_reflect(images: &Vec<u32>) -> usize {
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

fn one_bit_different(a: u32, b: u32) -> bool {
    let diff = a ^ b;
    diff & (diff - 1) == 0
}

fn try_reflect_smudged(images: &Vec<u32>) -> usize {
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
    let input = include_str!("../inputs/spring.txt");
    let matrices = input
        .split("\n\n")
        .map(|matrix_line| Matrix::parse(matrix_line))
        .collect::<Vec<Matrix>>();
   // .split("\n\n")
    // .map(|matrix_line| {
    //     let matrix = Matrix::parse(matrix_line);
    //     let vert = try_reflect(&matrix.cols);
    //     if vert > 0 {
    //         vert
    //     } else {
    //         let horiz = try_reflect(&matrix.rows);
    //         horiz * 100
    //     }
    // })
    // .sum::<usize>()
    0
}
