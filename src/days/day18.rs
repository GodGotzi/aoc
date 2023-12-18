use std::collections::{BinaryHeap, HashMap};

use priority_queue::PriorityQueue;

use crate::api::{Matrix2D, Solution};

struct Stroke {
    start: (i64, i64),
    end: (i64, i64),
    direction: char,
}

pub struct Day18;

impl Solution for Day18 {
    fn get_day(&self) -> u8 {
        18
    }

    fn part1(&self, input: String) -> String {
        let mut path_map: HashMap<(i32, i32), char> = HashMap::<(i32, i32), char>::new();

        let mut current = (0, 0);
        let mut min: Option<(i32, i32)> = None;
        let mut max: Option<(i32, i32)> = None;

        for line in input.lines() {
            let splitted = line.split(' ').collect::<Vec<&str>>();
            //let color = splitted[splitted.len() - 1].replace(['(', ')'], "");

            let count = splitted[1].parse::<usize>().unwrap();

            for _ in 0..count {
                match splitted[0].chars().next().unwrap() {
                    'R' => current.0 += 1,
                    'L' => current.0 -= 1,
                    'U' => current.1 += 1,
                    'D' => current.1 -= 1,
                    _ => panic!("Unknown direction: {}", splitted[0].chars().next().unwrap()),
                }

                if let Some((x, y)) = min {
                    min = Some((x.min(current.0), y.min(current.1)));
                } else {
                    min = Some(current);
                }

                if let Some((x, y)) = max {
                    max = Some((x.max(current.0), y.max(current.1)));
                } else {
                    max = Some(current);
                }

                path_map.insert(current, '#');
            }
        }

        let mut grid = Matrix2D::<char>::new(
            '.',
            (
                (max.unwrap().0 - min.unwrap().0) as usize + 3,
                (max.unwrap().1 - min.unwrap().1) as usize + 3,
            ),
        );

        for (pos, val) in path_map.iter() {
            if let Some(value) =
                grid.get_mut(&(pos.0 - min.unwrap().0 + 1, pos.1 - min.unwrap().1 + 1))
            {
                *value = *val;
            }
        }

        flood_fill(&mut grid, (0, 0));

        let mut inner_counter = 0;

        for val in grid.iter() {
            if *val != 'O' {
                inner_counter += 1;
            }
        }

        inner_counter.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut paths = Vec::new();

        let mut current = (0, 0);

        for line in input.lines() {
            let splitted = line.split(' ').collect::<Vec<&str>>();
            let color = splitted[splitted.len() - 1].replace(['(', ')', '#'], "");

            let count = usize::from_str_radix(&color, 16).unwrap();

            let new_current = match splitted[0].chars().next().unwrap() {
                'R' => (current.0 + count as i64, current.1),
                'L' => (current.0 - count as i64, current.1),
                'U' => (current.0, current.1 + count as i64),
                'D' => (current.0, current.1 - count as i64),
                _ => panic!("Unknown direction: {}", splitted[0].chars().next().unwrap()),
            };

            paths.push(Stroke {
                start: current,
                end: new_current,
                direction: splitted[0].chars().next().unwrap(),
            });

            current = new_current;
        }

        let mut queue = PriorityQueue::new();

        "".to_string()
    }
}

fn flood_fill(grid: &mut Matrix2D<char>, (row, col): (i32, i32)) {
    if grid.get(&(row, col)) == Some(&'.') {
        grid.set(&(row, col), 'O');

        flood_fill(grid, (row + 1, col));
        flood_fill(grid, (row - 1, col));
        flood_fill(grid, (row, col + 1));
        flood_fill(grid, (row, col - 1));
    }
}
