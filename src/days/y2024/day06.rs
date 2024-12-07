use crate::api::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn get_day(&self) -> u8 {
        6
    }

    fn get_year(&self) -> u16 {
        2024
    }

    fn part1(&self, input: String) -> String {
        let mut matrix = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let start = matrix
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, &cell)| {
                    if cell == '^' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        let mut velocity: (i32, i32) = (0, 1);
        let mut position: (i32, i32) = start;
        let mut visited = vec![position];

        while let Some(cell) = matrix
            .get((position.1 - velocity.1) as usize)
            .and_then(|row| row.get((position.0 + velocity.0) as usize))
        {
            velocity = if *cell == '#' {
                turn_90(velocity)
            } else {
                velocity
            };

            position = (position.0 + velocity.0, position.1 - velocity.1);

            if !visited.contains(&position) {
                visited.push(position);
            }
        }

        visited.len().to_string()
    }

    fn part2(&self, input: String) -> String {
        "".to_string()
    }
}

fn turn_90(velocity: (i32, i32)) -> (i32, i32) {
    match velocity {
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        _ => unreachable!(),
    }
}
