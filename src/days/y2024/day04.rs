use crate::api::Solution;
use itertools::Itertools;
use regex::Regex;

pub struct Day04;

impl Solution for Day04 {
    fn get_day(&self) -> u8 {
        4
    }

    fn get_year(&self) -> u16 {
        2024
    }

    fn part1(&self, input: String) -> String {
        let mut matrix = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut sum = 0;

        sum += input.matches("XMAS").count();
        sum += input.matches("SAMX").count();

        let rotated_input = rotate_input(&matrix);

        sum += rotated_input.matches("XMAS").count();
        sum += rotated_input.matches("SAMX").count();

        let dia_input = diagonals(&matrix).join("\n");

        sum += dia_input.matches("XMAS").count();
        sum += dia_input.matches("SAMX").count();

        matrix.reverse();

        let dia_input = diagonals(&matrix).join("\n");

        sum += dia_input.matches("XMAS").count();
        sum += dia_input.matches("SAMX").count();

        sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let matrix = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let regexes = [
            [
                Regex::new(r"M.S").unwrap(),
                Regex::new(r".A.").unwrap(),
                Regex::new(r"M.S").unwrap(),
            ],
            [
                Regex::new(r"S.M").unwrap(),
                Regex::new(r".A.").unwrap(),
                Regex::new(r"S.M").unwrap(),
            ],
            [
                Regex::new(r"M.M").unwrap(),
                Regex::new(r".A.").unwrap(),
                Regex::new(r"S.S").unwrap(),
            ],
            [
                Regex::new(r"S.S").unwrap(),
                Regex::new(r".A.").unwrap(),
                Regex::new(r"M.M").unwrap(),
            ],
            [
                Regex::new(r"S.M").unwrap(),
                Regex::new(r".A.").unwrap(),
                Regex::new(r"M.S").unwrap(),
            ],
            [
                Regex::new(r"M.S").unwrap(),
                Regex::new(r".A.").unwrap(),
                Regex::new(r"S.M").unwrap(),
            ],
        ];

        let mut count = 0;
        for row in 0..matrix.len() - 2 {
            for col in 0..matrix[row].len() - 2 {
                let block = matrix[row..row + 3]
                    .iter()
                    .map(|row| row[col..col + 3].iter().collect::<String>())
                    .collect::<Vec<String>>();

                if regexes
                    .iter()
                    .any(|block_regexes| (0..3).all(|i| block_regexes[i].is_match(&block[i])))
                {
                    count += 1;
                }
            }
        }

        count.to_string()
    }
}

fn rotate_input(matrix: &[Vec<char>]) -> String {
    let mut rotated_matrix = vec![vec![' '; matrix.len()]; matrix[0].len()];

    matrix.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &cell)| {
            rotated_matrix[j][matrix.len() - 1 - i] = cell;
        });
    });

    rotated_matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n")
}

fn diagonals(matrix: &[Vec<char>]) -> Vec<String> {
    let mut diagonals = vec![];

    for i in 0..matrix.len() {
        let mut diagonal = String::new();

        for j in 0..matrix[0].len() {
            if i + j < matrix.len() {
                diagonal.push(matrix[i + j][j]);
            }
        }

        diagonals.push(diagonal);
    }

    for j in 1..matrix[0].len() {
        let mut diagonal = String::new();

        for i in 0..matrix.len() {
            if i + j < matrix[0].len() {
                diagonal.push(matrix[i][i + j]);
            }
        }

        diagonals.push(diagonal);
    }

    diagonals
}
