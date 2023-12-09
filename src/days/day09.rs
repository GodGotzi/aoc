use crate::api::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn get_day(&self) -> u8 {
        9
    }

    fn part1(&self, input: String) -> String {
        let mut sum = 0;

        //println!("hello {}", input);

        for line in input.lines() {
            let values = line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            sum += next(&values) + values[values.len() - 1];
        }

        sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut sum = 0;

        //println!("hello {}", input);

        for line in input.lines() {
            let values = line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            sum += values[0] - previous(&values);
        }

        sum.to_string()
    }
}

pub fn next(values: &[i32]) -> i32 {
    let mut diffs = Vec::new();

    for index in 0..values.len() - 1 {
        diffs.push(values[index + 1] - values[index]);
    }

    if diffs.iter().all(|diff| *diff == diffs[0]) {
        return diffs[diffs.len() - 1];
    }

    diffs[diffs.len() - 1] + next(&diffs)
}

pub fn previous(values: &[i32]) -> i32 {
    let mut diffs = Vec::new();

    for index in 0..values.len() - 1 {
        diffs.push(values[index + 1] - values[index]);
    }

    if diffs.iter().all(|diff| *diff == diffs[0]) {
        return diffs[0];
    }

    diffs[0] - previous(&diffs)
}
