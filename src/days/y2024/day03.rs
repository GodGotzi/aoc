use crate::api::Solution;

use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    fn get_day(&self) -> u8 {
        3
    }

    fn get_year(&self) -> u16 {
        2024
    }

    fn part1(&self, input: String) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut sum = 0;

        re.captures_iter(&input).for_each(|cap| {
            let a = cap[1].parse::<u32>().unwrap();
            let b = cap[2].parse::<u32>().unwrap();

            sum += a * b;
        });

        sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let dos = Regex::new(r"do()").unwrap();
        let donts = Regex::new(r"don't()").unwrap();

        let mut dos_vec = vec![];
        let mut donts_vec = vec![];

        for cap in dos.captures_iter(&input) {
            if let Some(matched) = cap.get(0) {
                dos_vec.push(matched.end());
            }
        }

        for cap in donts.captures_iter(&input) {
            if let Some(matched) = cap.get(0) {
                donts_vec.push(matched.end());
            }
        }

        let mut sum = 0;

        for cap in re.captures_iter(&input) {
            if let Some(matched) = cap.get(0) {
                let mut dos_last = true;

                for i in (0..matched.start()).rev() {
                    if dos_vec.contains(&i) {
                        dos_last = true;
                        break;
                    }

                    if donts_vec.contains(&i) {
                        dos_last = false;
                        break;
                    }
                }

                if dos_last {
                    let a = cap[1].parse::<u32>().unwrap();
                    let b = cap[2].parse::<u32>().unwrap();

                    sum += a * b;
                }
            }
        }

        sum.to_string()
    }
}
