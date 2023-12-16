use crate::api::{Matrix2D, Solution};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct Day16;

impl Solution for Day16 {
    fn get_day(&self) -> u8 {
        16
    }

    fn part1(&self, input: String) -> String {
        let contraption = Matrix2D::from(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        );

        let mut visited = Matrix2D::new(
            Vec::<Direction>::new(),
            (contraption.rows(), contraption.cols()),
        );

        move_beam(&contraption, &mut visited, (0, 0), Direction::Right);

        visited
            .iter()
            .filter(|directions| !directions.is_empty())
            .count()
            .to_string()
    }

    fn part2(&self, input: String) -> String {
        let contraption = Matrix2D::from(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        );

        let mut largest_energized = 0;

        for col in 0..contraption.cols() {
            let energized = compute_contraption(&contraption, (0, col as i32), Direction::Down);

            if energized > largest_energized {
                largest_energized = energized;
            }
        }

        for col in 0..contraption.cols() {
            let energized = compute_contraption(
                &contraption,
                (contraption.rows() as i32 - 1, col as i32),
                Direction::Down,
            );

            if energized > largest_energized {
                largest_energized = energized;
            }
        }

        for row in 0..contraption.rows() {
            let energized = compute_contraption(&contraption, (row as i32, 0), Direction::Right);

            if energized > largest_energized {
                largest_energized = energized;
            }
        }

        for row in 0..contraption.rows() {
            let energized = compute_contraption(
                &contraption,
                (row as i32, contraption.cols() as i32 - 1),
                Direction::Left,
            );

            if energized > largest_energized {
                largest_energized = energized;
            }
        }

        largest_energized.to_string()
    }
}

fn compute_contraption(
    contraption: &Matrix2D<char>,
    start: (i32, i32),
    direction: Direction,
) -> usize {
    let mut visited = Matrix2D::new(
        Vec::<Direction>::new(),
        (contraption.rows(), contraption.cols()),
    );

    move_beam(contraption, &mut visited, start, direction);

    visited
        .iter()
        .filter(|directions| !directions.is_empty())
        .count()
}

fn move_beam(
    contraption: &Matrix2D<char>,
    visited: &mut Matrix2D<Vec<Direction>>,
    pos: (i32, i32),
    direction: Direction,
) {
    if let Some(tile) = contraption.get(&(pos.0, pos.1)) {
        if let Some(directions) = visited.get_mut(&(pos.0, pos.1)) {
            if !directions.contains(&direction) {
                directions.push(direction.clone());
            } else {
                return;
            }
        }

        match direction {
            Direction::Up => match tile {
                &'.' | &'|' => move_beam(contraption, visited, (pos.0 - 1, pos.1), direction),
                &'-' => {
                    move_beam(contraption, visited, (pos.0, pos.1 - 1), Direction::Left);
                    move_beam(contraption, visited, (pos.0, pos.1 + 1), Direction::Right);
                }
                &'/' => move_beam(contraption, visited, (pos.0, pos.1 + 1), Direction::Right),
                &'\\' => move_beam(contraption, visited, (pos.0, pos.1 - 1), Direction::Left),
                _ => panic!("Invalid tile"),
            },
            Direction::Down => match tile {
                &'.' | &'|' => move_beam(contraption, visited, (pos.0 + 1, pos.1), direction),
                &'-' => {
                    move_beam(contraption, visited, (pos.0, pos.1 - 1), Direction::Left);
                    move_beam(contraption, visited, (pos.0, pos.1 + 1), Direction::Right);
                }
                &'/' => move_beam(contraption, visited, (pos.0, pos.1 - 1), Direction::Left),
                &'\\' => move_beam(contraption, visited, (pos.0, pos.1 + 1), Direction::Right),
                _ => panic!("Invalid tile"),
            },
            Direction::Right => match tile {
                &'.' | &'-' => move_beam(contraption, visited, (pos.0, pos.1 + 1), direction),
                &'|' => {
                    move_beam(contraption, visited, (pos.0 - 1, pos.1), Direction::Up);
                    move_beam(contraption, visited, (pos.0 + 1, pos.1), Direction::Down);
                }
                &'/' => move_beam(contraption, visited, (pos.0 - 1, pos.1), Direction::Up),
                &'\\' => move_beam(contraption, visited, (pos.0 + 1, pos.1), Direction::Down),
                _ => panic!("Invalid tile"),
            },
            Direction::Left => match tile {
                &'.' | &'-' => move_beam(contraption, visited, (pos.0, pos.1 - 1), direction),
                &'|' => {
                    move_beam(contraption, visited, (pos.0 - 1, pos.1), Direction::Up);
                    move_beam(contraption, visited, (pos.0 + 1, pos.1), Direction::Down);
                }
                &'/' => move_beam(contraption, visited, (pos.0 + 1, pos.1), Direction::Down),
                &'\\' => move_beam(contraption, visited, (pos.0 - 1, pos.1), Direction::Up),
                _ => panic!("Invalid tile"),
            },
        }
    }
}
