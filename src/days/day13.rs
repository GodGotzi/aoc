use itertools::Itertools;

use crate::api::{IndexString, Matrix2D, Solution};

pub struct Day13;

impl Solution for Day13 {
    fn get_day(&self) -> u8 {
        13
    }

    fn part1(&self, input: String) -> String {
        let patterns = parse_patterns(&input);

        let mut sum = 0;
        for pattern in patterns {
            for index in 1..pattern.rows() {
                if let Some(col) = mirror_columns(&pattern, index) {
                    sum += col;
                    break;
                } else if let Some(row) = mirror_rows(&pattern, index) {
                    sum += row * 100;
                    break;
                }
            }
        }

        sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut patterns = parse_patterns(&input);

        let mut sum = 0;
        for pattern in patterns.iter_mut() {
            println!("{:?}", pattern);

            if let Some(col) = find_smudge_cols(pattern) {
                sum += col;
                break;
            } else if let Some(row) = find_smudge_rows(pattern) {
                sum += row * 100;
                break;
            }

            println!("{}", sum);
        }

        sum.to_string()
    }
}

fn find_smudge_rows(matrix: &Matrix2D<char>) -> Option<usize> {
    for row in 1..matrix.rows() {
        let distance = row.min(matrix.rows() - row);
        if (0..=distance)
            .map(|pos| (compare_rows(matrix, row - pos, row + pos - 1), pos))
            .any(|(diff, _)| diff.len() == 1)
        {
            return Some(row);
        }
    }

    None
}

fn find_smudge_cols(matrix: &Matrix2D<char>) -> Option<usize> {
    for col in 1..matrix.cols() {
        let distance = col.min(matrix.cols() - col);
        if (0..=distance)
            .map(|pos| (compare_cols(matrix, col - pos, col + pos - 1), pos))
            .any(|(diff, _)| diff.len() == 1)
        {
            return Some(col);
        }
    }

    None
}

fn compare_rows(matrix: &Matrix2D<char>, row_a: usize, row_b: usize) -> Vec<usize> {
    let mut invalid = Vec::new();

    let row_a_str = matrix.get_row_into_string(row_a).unwrap();
    let row_b_str = matrix.get_row_into_string(row_b).unwrap();

    for index in 0..row_a_str.len() {
        if row_a_str.index(index) != row_b_str.index(index) {
            invalid.push(index);
        }
    }

    invalid
}

fn compare_cols(matrix: &Matrix2D<char>, col_a: usize, col_b: usize) -> Vec<usize> {
    let mut invalid = Vec::new();

    let col_a_str = matrix.get_col_into_string(col_a).unwrap();
    let col_b_str = matrix.get_col_into_string(col_b).unwrap();

    for index in 0..col_a_str.len() {
        if col_a_str.index(index) != col_b_str.index(index) {
            invalid.push(index);
        }
    }

    invalid
}

fn mirror_columns(matrix: &Matrix2D<char>, col: usize) -> Option<usize> {
    let distance = col.min(matrix.cols() - col);

    if (1..=distance).all(|pos| {
        matrix.get_col_into_string(col - pos).unwrap()
            == matrix.get_col_into_string(col + pos - 1).unwrap()
    }) {
        return Some(col);
    }

    None
}

fn mirror_rows(matrix: &Matrix2D<char>, row: usize) -> Option<usize> {
    let distance = row.min(matrix.rows() - row);

    if (1..=distance).all(|pos| {
        matrix.get_row_into_string(row - pos).unwrap()
            == matrix.get_row_into_string(row + pos - 1).unwrap()
    }) {
        return Some(row);
    }

    None
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
