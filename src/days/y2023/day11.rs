use std::collections::HashSet;

use crate::api::{Matrix2D, Solution};

pub struct Day11;

impl Solution for Day11 {
    fn get_day(&self) -> u8 {
        11
    }

    fn part1(&self, input: String) -> String {
        let grid: Matrix2D<char> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
            .into();

        compute_distances(&grid, 2).to_string()
    }

    fn part2(&self, input: String) -> String {
        let grid: Matrix2D<char> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
            .into();

        compute_distances(&grid, 1000000).to_string()
    }
}

fn compute_distances(grid: &Matrix2D<char>, emptyness_multiplier: usize) -> usize {
    let (expanded_rows, expanded_cols) = get_expanded_positions(grid);

    let galaxies = grid
        .iter_enumerate()
        .filter(|(_, value)| **value == '#')
        .map(|(pos, _)| pos)
        .collect::<Vec<(usize, usize)>>();

    let mut pairs = Vec::new();

    for x in 0..galaxies.len() {
        for y in 0..galaxies.len() {
            if !pairs.contains(&[galaxies[x], galaxies[y]])
                && !pairs.contains(&[galaxies[y], galaxies[x]])
            {
                pairs.push([galaxies[x], galaxies[y]]);
            }
        }
    }

    let mut sum = 0;

    for pair in pairs {
        let mut queue: Vec<(usize, (usize, usize))> = vec![(0, pair[0])];
        let mut visited = vec![pair[0]];

        while let Some((counter, pos)) = queue.pop() {
            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

            if pos == pair[1] {
                sum += counter;
                break;
            }

            let mut smallest_dist: Option<(usize, f32, (usize, usize))> = None;

            for direction in directions {
                let new_pos = (pos.0 as i32 + direction.0, pos.1 as i32 + direction.1);

                if grid.get(&new_pos).is_some() {
                    let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

                    if !visited.contains(&new_pos) {
                        let dist = distance(new_pos, pair[1]);

                        if smallest_dist.is_none() || smallest_dist.unwrap().1 > dist {
                            if (expanded_rows.contains(&new_pos.0) && direction.0 != 0)
                                || (expanded_cols.contains(&new_pos.1) && direction.1 != 0)
                            {
                                smallest_dist =
                                    Some((counter + emptyness_multiplier, dist, new_pos));
                            } else {
                                smallest_dist = Some((counter + 1, dist, new_pos));
                            }
                        }

                        visited.push(new_pos);
                    }
                }
            }

            let (counter, _, pos) = smallest_dist.unwrap();
            queue.push((counter, pos));
        }
    }

    sum
}

fn distance(pos1: (usize, usize), pos2: (usize, usize)) -> f32 {
    ((pos1.0 as i32 - pos2.0 as i32).pow(2) as f32 + (pos1.1 as i32 - pos2.1 as i32).pow(2) as f32)
        .sqrt()
}

fn get_expanded_positions(grid: &Matrix2D<char>) -> (HashSet<usize>, HashSet<usize>) {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();

    for row in 0..grid.rows() {
        if (0..grid.cols()).all(|col| *grid.get(&(row as i32, col as i32)).unwrap() != '#') {
            rows.insert(row);
        }
    }

    for col in 0..grid.cols() {
        if (0..grid.rows()).all(|row| *grid.get(&(row as i32, col as i32)).unwrap() != '#') {
            cols.insert(col);
        }
    }

    (rows, cols)
}
