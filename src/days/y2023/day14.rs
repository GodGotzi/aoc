use crate::api::{Matrix2D, Solution};

pub struct Day14;

impl Solution for Day14 {
    fn get_day(&self) -> u8 {
        14
    }

    fn part1(&self, input: String) -> String {
        let mut platform = Matrix2D::from(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        );

        let rolling = platform
            .iter_enumerate()
            .filter(|((_, _), pos)| **pos == 'O')
            .map(|((row, col), _)| (row, col))
            .collect::<Vec<(usize, usize)>>();

        tilt(&mut platform, &rolling, -1, 0);

        sum_total_load(&platform).to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut platform = Matrix2D::from(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        );

        let mut states: Vec<(Vec<(usize, usize)>, usize)> = Vec::new();

        let mut rolling = platform
            .iter_enumerate()
            .filter(|((_, _), pos)| **pos == 'O')
            .map(|((row, col), _)| (row, col))
            .collect::<Vec<(usize, usize)>>();

        let mut cycle_count = 1;
        let mut rep: Option<usize> = None;

        while rep.is_none() {
            cycle(&mut platform, &mut rolling);

            if let Some((_, old_cycle_count)) = states.iter().find(|(state, _)| *state == rolling) {
                //println!("{} {}", cycle_count, old_cycle_count);
                rep = Some(*old_cycle_count);
                break;
            } else {
                states.push((rolling.clone(), cycle_count));
            }

            cycle_count += 1;
        }

        let diff = cycle_count - rep.unwrap();

        for _ in 0..((1_000_000_000 - cycle_count) % diff) {
            cycle(&mut platform, &mut rolling);
        }

        sum_total_load(&platform).to_string()
    }
}

fn cycle(platform: &mut Matrix2D<char>, rolling: &mut Vec<(usize, usize)>) {
    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    for direction in directions.iter() {
        tilt(platform, rolling, direction.0, direction.1);

        *rolling = platform
            .iter_enumerate()
            .filter(|((_, _), pos)| **pos == 'O')
            .map(|((row, col), _)| (row, col))
            .collect::<Vec<(usize, usize)>>();
    }
}

fn sum_total_load(platform: &Matrix2D<char>) -> usize {
    platform
        .iter_enumerate()
        .filter(|((_, _), pos)| **pos == 'O')
        .map(|((row, _), _)| platform.rows() - row)
        .sum::<usize>()
}

fn tilt(platform: &mut Matrix2D<char>, rolling: &[(usize, usize)], row_diff: i32, col_diff: i32) {
    for (row, col) in rolling {
        if platform.get(&(*row as i32, *col as i32)) == Some(&'O') {
            platform.set(&(*row as i32, *col as i32), '.');

            roll(platform, *row as i32, *col as i32, row_diff, col_diff);
        }
    }
}

fn roll(platform: &mut Matrix2D<char>, row: i32, col: i32, row_diff: i32, col_diff: i32) {
    if let Some(val) = platform.get(&(row + row_diff, col + col_diff)) {
        if *val == 'O' {
            platform.set(&(row + row_diff, col + col_diff), '.');
            roll(platform, row + row_diff, col + col_diff, row_diff, col_diff);

            if platform.get(&(row + row_diff, col + col_diff)) == Some(&'.') {
                roll(platform, row, col, row_diff, col_diff);
            } else {
                platform.set(&(row, col), 'O');
            }
        } else if *val == '.' {
            roll(platform, row + row_diff, col + col_diff, row_diff, col_diff);
        } else {
            platform.set(&(row, col), 'O');
        }
    } else {
        platform.set(&(row, col), 'O');
    }
}
