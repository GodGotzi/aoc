use itertools::Itertools;

use crate::api::{Matrix2D, Solution};

pub struct Day13;

impl Solution for Day13 {
    fn get_day(&self) -> u8 {
        13
    }

    fn part1(&self, input: String) -> String {
        let patterns = parse_patterns(&input);

        patterns
            .iter()
            .map(|pattern| {
                if let Some(col) = find_mirror_cols(pattern, 0) {
                    col
                } else {
                    let row = find_mirror_rows(pattern, 0).unwrap();
                    row * 100
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: String) -> String {
        let patterns = parse_patterns(&input);

        patterns
            .iter()
            .map(|pattern| {
                if let Some(col) = find_mirror_cols(pattern, 1) {
                    col
                } else {
                    let row = find_mirror_rows(pattern, 1).expect("No mirror found");
                    row * 100
                }
            })
            .sum::<usize>()
            .to_string()
    }
}

fn find_mirror_cols(matrix: &Matrix2D<char>, smudge: usize) -> Option<usize> {
    for col in 0..matrix.cols() - 1 {
        let distance = (col + 1).min(matrix.cols() - col - 1);

        let count = (0..distance)
            .map(|pos| diff_cols(matrix, col - pos, col + pos + 1))
            .sum::<usize>();

        if count == smudge {
            return Some(col + 1);
        }
    }

    None
}

fn find_mirror_rows(matrix: &Matrix2D<char>, smudge: usize) -> Option<usize> {
    for row in 0..matrix.rows() - 1 {
        let distance = (row + 1).min(matrix.rows() - row - 1);
        let count = (0..distance)
            .map(|pos| diff_rows(matrix, row - pos, row + pos + 1))
            .sum::<usize>();

        if count == smudge {
            return Some(row + 1);
        }
    }

    None
}

fn diff_rows(matrix: &Matrix2D<char>, row_a: usize, row_b: usize) -> usize {
    let row_a_str = matrix.get_row(row_a).unwrap();
    let row_b_str = matrix.get_row(row_b).unwrap();

    row_a_str
        .iter()
        .zip(row_b_str.iter())
        .filter(|(a, b)| a != b)
        .count()
}

fn diff_cols(matrix: &Matrix2D<char>, col_a: usize, col_b: usize) -> usize {
    let col_a_str = matrix.get_col(col_a).unwrap();
    let col_b_str = matrix.get_col(col_b).unwrap();

    col_a_str
        .iter()
        .zip(col_b_str.iter())
        .filter(|(a, b)| a != b)
        .count()
}

fn parse_patterns(input: &str) -> Vec<Matrix2D<char>> {
    input
        .split("\r\n\r\n")
        .map(|pattern| {
            Matrix2D::from(
                pattern
                    .lines()
                    .map(|line| line.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>(),
            )
        })
        .collect_vec()
}
