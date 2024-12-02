use itertools::Itertools;

use crate::api::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn get_day(&self) -> u8 {
        2
    }

    fn get_year(&self) -> u16 {
        2024
    }

    fn part1(&self, input: String) -> String {
        let mut unusual_data = vec![];

        for line in input.lines() {
            let report = line
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec();

            unusual_data.push(report);
        }

        let mut safe = 0;

        for report in unusual_data {
            if is_safe(&report) {
                safe += 1;
            }
        }

        safe.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut unusual_data = vec![];

        for line in input.lines() {
            let report = line
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec();

            unusual_data.push(report);
        }

        let mut safe = 0;

        for report in unusual_data {
            if is_safe(&report) {
                safe += 1;
            } else {
                for i in 0..report.len() {
                    let mut report = report.clone();

                    report.remove(i);

                    if is_safe(&report) {
                        safe += 1;
                        break;
                    }
                }
            }
        }

        safe.to_string()
    }
}

fn is_safe(report: &[u32]) -> bool {
    let mut last = report[0];

    let mut increasing = 0;
    let mut decreasing = 0;

    for &current in report.iter().skip(1) {
        if current > last && current - last <= 3 {
            increasing += 1;
        } else if current < last && last - current <= 3 {
            decreasing += 1;
        } else {
            break;
        }

        last = current;
    }

    increasing == report.len() - 1 || decreasing == report.len() - 1
}
