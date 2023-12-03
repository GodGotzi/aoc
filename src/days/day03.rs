use std::collections::HashMap;

use crate::api::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn get_day(&self) -> u8 {
        3
    }

    fn part1(&self, input: String) -> String {
        let matrix: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut sum = 0;

        let mut row = 0;

        while row < matrix.len() {
            let mut colum = 0;

            while colum < matrix[row].len() {
                if matrix[row][colum].is_numeric() {
                    let mut part_number: String = String::from("");
                    let mut next = true;
                    let mut has_neighbors = false;
                    //let mut gear_map = HashMap::new();

                    while colum < matrix[row].len() && next {
                        let neighbors = get_neighbors(&matrix, row, colum);
                        next = has_next(&matrix, row, colum);

                        part_number.push(matrix[row][colum]);

                        if !has_neighbors && !neighbors.is_empty() {
                            has_neighbors = true;
                        }

                        if next {
                            colum += 1;
                        }
                    }

                    if has_neighbors {
                        sum += part_number.parse::<u32>().unwrap();
                    }
                }

                colum += 1;
            }

            row += 1;
        }

        sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let matrix: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        let mut row = 0;

        while row < matrix.len() {
            let mut colum = 0;

            while colum < matrix[row].len() {
                if matrix[row][colum].is_numeric() {
                    let mut part_number: String = String::from("");
                    let mut next = true;
                    let mut has_neighbors = false;
                    let mut part_gear_map: HashMap<(usize, usize), usize> = HashMap::new();

                    while colum < matrix[row].len() && next {
                        let neighbors = get_neighbors(&matrix, row, colum);
                        next = has_next(&matrix, row, colum);

                        for neighbor in neighbors.iter() {
                            if matrix[neighbor.0][neighbor.1] == '*' {
                                part_gear_map.insert(*neighbor, 1);
                            }
                        }

                        part_number.push(matrix[row][colum]);

                        if !has_neighbors && !neighbors.is_empty() {
                            has_neighbors = true;
                        }

                        if next {
                            colum += 1;
                        }
                    }

                    if has_neighbors {
                        for gear in part_gear_map.keys() {
                            if let Some(vec) = gear_map.get_mut(gear) {
                                vec.push(part_number.parse::<u32>().unwrap());
                            } else {
                                gear_map.insert(*gear, vec![part_number.parse::<u32>().unwrap()]);
                            }
                        }
                    }
                }

                colum += 1;
            }

            row += 1;
        }

        let mut sum = 0;

        for (_gear, part_numbers) in gear_map
            .iter()
            .filter(|(_, part_numbers)| part_numbers.len() == 2)
        {
            sum += part_numbers[0] * part_numbers[1];
        }

        sum.to_string()
    }
}

pub fn has_next(matrix: &[Vec<char>], row: usize, colum: usize) -> bool {
    if let Some(rows) = matrix.get(row) {
        if let Some(value) = rows.get(colum + 1) {
            if value.is_numeric() {
                return true;
            }
        }
    }

    false
}

pub fn get_neighbors(matrix: &[Vec<char>], row: usize, colum: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if let Some(rows) = matrix.get(row) {
        if let Some(value) = rows.get(colum + 1) {
            if value != &'.' && !value.is_numeric() {
                neighbors.push((row, colum + 1));
            }
        }
    }

    if let Some(rows) = matrix.get(row) {
        if let Some(value) = rows.get(colum - 1) {
            if value != &'.' && !value.is_numeric() {
                neighbors.push((row, colum - 1));
            }
        }
    }

    if let Some(rows) = matrix.get(row - 1) {
        if let Some(value) = rows.get(colum) {
            if value != &'.' && !value.is_numeric() {
                neighbors.push((row - 1, colum));
            }
        }
    }

    if let Some(rows) = matrix.get(row + 1) {
        if let Some(value) = rows.get(colum) {
            if value != &'.' && !value.is_numeric() {
                neighbors.push((row + 1, colum));
            }
        }
    }

    //diagonals
    if let Some(rows) = matrix.get(row - 1) {
        if let Some(value) = rows.get(colum - 1) {
            if value != &'.' && !value.is_numeric() {
                neighbors.push((row - 1, colum - 1));
            }
        }
    }

    if let Some(rows) = matrix.get(row - 1) {
        if let Some(value) = rows.get(colum + 1) {
            if value != &'.' && !value.is_numeric() {
                neighbors.push((row - 1, colum + 1));
            }
        }
    }

    if let Some(rows) = matrix.get(row + 1) {
        if let Some(value) = rows.get(colum - 1) {
            if value != &'.' && !value.is_numeric() {
                neighbors.push((row + 1, colum - 1));
            }
        }
    }

    if let Some(rows) = matrix.get(row + 1) {
        if let Some(value) = rows.get(colum + 1) {
            if value != &'.' && !value.is_numeric() {
                neighbors.push((row + 1, colum + 1));
            }
        }
    }

    neighbors
}
