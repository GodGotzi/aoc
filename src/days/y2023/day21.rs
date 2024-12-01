use crate::api::{Matrix2D, Solution};

pub struct Day21;

impl Solution for Day21 {
    fn get_day(&self) -> u8 {
        21
    }

    fn part1(&self, input: String) -> String {
        let grid = Matrix2D::from(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        );

        let (start_row, start_col) = grid
            .iter_enumerate()
            .find(|(_, value)| **value == 'S')
            .map(|(pos, _)| pos)
            .unwrap();

        let mut finished = Vec::new();

        move_elf(
            &grid,
            &mut finished,
            (0, (start_row as i32, start_col as i32)),
            64,
        );

        finished.len().to_string()
    }

    fn part2(&self, input: String) -> String {
        "".to_string()
    }
}

fn move_elf(
    grid: &Matrix2D<char>,
    finished: &mut Vec<(usize, usize)>,
    (current_steps, (row, col)): (usize, (i32, i32)),
    steps: usize,
) {
    if current_steps == steps {
        if !finished.contains(&(row as usize, col as usize)) {
            finished.push((row as usize, col as usize));
        }
        return;
    }

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    for direction in directions {
        if let Some(value) = grid.get(&(row + direction.0, col + direction.1)) {
            if *value == '.' {
                move_elf(
                    grid,
                    finished,
                    (current_steps + 1, (row + direction.0, col + direction.1)),
                    steps,
                )
            }
        }
    }
}
