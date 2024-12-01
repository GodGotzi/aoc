use phf::phf_map;

use crate::api::{Matrix2D, Solution};

static slides: phf::Map<char, (i32, i32)> = phf_map! {
    '>' => (0, 1),
    '<' => (0, -1),
    '^' => (-1, 0),
    'v' => (1, 0),
};

pub struct Day23;

impl Solution for Day23 {
    fn get_day(&self) -> u8 {
        23
    }

    fn part1(&self, input: String) -> String {
        let hiking_map = Matrix2D::from(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        );

        let start: (i32, i32) = (
            0,
            input
                .lines()
                .next()
                .unwrap()
                .chars()
                .enumerate()
                .find(|(_, c)| *c == '.')
                .unwrap()
                .0 as i32,
        );

        let end: (i32, i32) = (
            input.lines().count() as i32 - 1,
            input
                .lines()
                .last()
                .unwrap()
                .chars()
                .enumerate()
                .find(|(_, c)| *c == '.')
                .unwrap()
                .0 as i32,
        );

        let mut largest = None;

        let mut queue: Vec<((i32, i32), Vec<(i32, i32)>)> = vec![(start, Vec::new())];

        while let Some((current, visited)) = queue.pop() {
            if current == end {
                if largest.is_none() || visited.len() > largest.unwrap() {
                    largest = Some(visited.len());
                }

                continue;
            }

            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().filter(|dir| {
                if let Some(value) = hiking_map.get(&(current.0 + dir.0, current.1 + dir.1)) {
                    if !visited.contains(&(current.0 + dir.0, current.1 + dir.1)) {
                        if let Some(value) = slides.get(value) {
                            *value == (dir.0, dir.1)
                        } else {
                            *value == '.'
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            for direction in directions {
                let mut new_visited = visited.clone();
                new_visited.push((current.0 + direction.0, current.1 + direction.1));

                queue.push((
                    (current.0 + direction.0, current.1 + direction.1),
                    new_visited,
                ));
            }

            queue.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
        }

        largest.unwrap().to_string()
    }

    fn part2(&self, input: String) -> String {
        let input = input.replace(['>', '<', '^', 'v'], ".");

        let hiking_map = Matrix2D::from(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        );

        let start: (i32, i32) = (
            0,
            input
                .lines()
                .next()
                .unwrap()
                .chars()
                .enumerate()
                .find(|(_, c)| *c == '.')
                .unwrap()
                .0 as i32,
        );

        let end: (i32, i32) = (
            input.lines().count() as i32 - 1,
            input
                .lines()
                .last()
                .unwrap()
                .chars()
                .enumerate()
                .find(|(_, c)| *c == '.')
                .unwrap()
                .0 as i32,
        );

        let mut largest = None;

        let mut queue: Vec<((i32, i32), Vec<(i32, i32)>)> = vec![(start, Vec::new())];

        while let Some((current, visited)) = queue.pop() {
            if current == end {
                if largest.is_none() || visited.len() > largest.unwrap() {
                    largest = Some(visited.len());
                }

                println!("{:?}", largest);

                continue;
            }

            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().filter(|dir| {
                if let Some(value) = hiking_map.get(&(current.0 + dir.0, current.1 + dir.1)) {
                    if !visited.contains(&(current.0 + dir.0, current.1 + dir.1)) {
                        if let Some(value) = slides.get(value) {
                            *value == (dir.0, dir.1)
                        } else {
                            *value == '.'
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            for direction in directions {
                let mut new_visited = visited.clone();
                new_visited.push((current.0 + direction.0, current.1 + direction.1));

                queue.push((
                    (current.0 + direction.0, current.1 + direction.1),
                    new_visited,
                ));
            }

            queue.sort_by(|a, b| manhattan_distance(a.0, end).cmp(&manhattan_distance(b.0, end)));
        }

        largest.unwrap().to_string()
    }
}

fn manhattan_distance((row1, col1): (i32, i32), (row2, col2): (i32, i32)) -> i32 {
    (row1 - row2).abs() + (col1 - col2).abs()
}
