use std::collections::HashMap;

use itertools::Itertools;

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

        let mut buffer = "".to_string();
        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        for (row_index, row) in matrix.iter().enumerate() {
            for (colum_index, value) in row.iter().enumerate() {
                if value.is_numeric() {
                    buffer.push(*value);
                    neighbors.extend(get_neighbors(&matrix, row_index, colum_index));
                } else if let Ok(part_number) = buffer.parse::<u32>() {
                    if !neighbors.is_empty() {
                        sum += part_number;
                    }

                    buffer.clear();
                    neighbors.clear();
                }
            }
        }

        sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let matrix: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        let mut buffer = "".to_string();
        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        for (row_index, row) in matrix.iter().enumerate() {
            for (colum_index, value) in row.iter().enumerate() {
                if value.is_numeric() {
                    buffer.push(*value);
                    neighbors.extend(get_neighbors(&matrix, row_index, colum_index));
                } else if let Ok(part_number) = buffer.parse::<u32>() {
                    neighbors.iter().unique().for_each(|neighbor| {
                        if let Some(vec) = gear_map.get_mut(neighbor) {
                            vec.push(part_number);
                        } else {
                            gear_map.insert(*neighbor, vec![part_number]);
                        }
                    });

                    buffer.clear();
                    neighbors.clear();
                }
            }
        }

        gear_map
            .values()
            .filter(|part_numbers| part_numbers.len() == 2)
            .map(|part_numbers| part_numbers[0] * part_numbers[1])
            .sum::<u32>()
            .to_string()
    }
}

pub fn get_val(matrix: &[Vec<char>], row: usize, colum: usize) -> Option<char> {
    if let Some(rows) = matrix.get(row) {
        if let Some(value) = rows.get(colum) {
            return Some(*value);
        }
    }

    None
}

pub fn get_neighbors(matrix: &[Vec<char>], row: usize, colum: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let mut positions: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (-1, 0),
        (1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    for position in positions.iter_mut() {
        let row = (row as i32 + position.0) as usize;
        let colum = (colum as i32 + position.1) as usize;

        if let Some(value) = get_val(matrix, row, colum) {
            if value != '.' && !value.is_numeric() {
                neighbors.push((row, colum));
            }
        }
    }

    neighbors
}
