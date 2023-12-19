use std::collections::HashMap;

use crate::api::{Matrix2D, Solution};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Stroke {
    start: (i32, i32),
    end: (i32, i32),
}

pub struct Day18;

impl Solution for Day18 {
    fn get_day(&self) -> u8 {
        18
    }

    fn part1(&self, input: String) -> String {
        let mut paths = Vec::new();

        let mut current = (0, 0);
        let mut boundaries = 0;

        for line in input.lines() {
            let splitted = line.split(' ').collect::<Vec<&str>>();

            let count = splitted[1].parse::<usize>().unwrap();
            boundaries += count;

            let new_current = match splitted[0].chars().next().unwrap() {
                'R' => (current.0 + count as i32, current.1),
                'L' => (current.0 - count as i32, current.1),
                'U' => (current.0, current.1 + count as i32),
                'D' => (current.0, current.1 - count as i32),
                _ => panic!("Unknown direction: {}", splitted[0].chars().next().unwrap()),
            };

            paths.push(Stroke {
                start: current,
                end: new_current,
            });

            current = new_current;
        }

        //Greens Theorem
        let mut area: i64 = 0;

        for stroke in paths {
            area += stroke.start.0 as i64 * stroke.end.1 as i64
                - stroke.start.1 as i64 * stroke.end.0 as i64;
        }
        area = area.abs() / 2;

        //Pick's Theorem
        let interior = area as usize - boundaries / 2 + 1;

        (interior + boundaries).to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut paths = Vec::new();

        let mut current = (0, 0);
        let mut boundaries = 0;

        for line in input.lines() {
            let splitted = line.split(' ').collect::<Vec<&str>>();
            let color = splitted[splitted.len() - 1].replace(['(', ')', '#'], "");

            let count = usize::from_str_radix(&color[..color.len() - 1], 16).unwrap();
            boundaries += count;

            let dir = match color.chars().last().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("Unknown direction: {}", color.chars().last().unwrap()),
            };

            let new_current = match dir {
                'R' => (current.0 + count as i32, current.1),
                'L' => (current.0 - count as i32, current.1),
                'U' => (current.0, current.1 + count as i32),
                'D' => (current.0, current.1 - count as i32),
                _ => panic!("Unknown direction: {}", splitted[0].chars().next().unwrap()),
            };

            paths.push(Stroke {
                start: current,
                end: new_current,
            });

            current = new_current;
        }

        //Greens Theorem
        let mut area: i64 = 0;

        for stroke in paths {
            area += stroke.start.0 as i64 * stroke.end.1 as i64
                - stroke.start.1 as i64 * stroke.end.0 as i64;
        }
        area = area.abs() / 2;

        //Pick's Theorem
        let interior = area as usize - boundaries / 2 + 1;

        (interior + boundaries).to_string()
    }
}
