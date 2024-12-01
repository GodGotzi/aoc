use crate::api::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn get_day(&self) -> u8 {
        6
    }

    fn part1(&self, input: String) -> String {
        let lines = input.lines().collect::<Vec<&str>>();

        let time = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .filter(|s| !s.starts_with(' ') && !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let distance = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .filter(|s| !s.starts_with(' ') && !s.is_empty())
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut product = 1;

        for index in 0..time.len() {
            let delta = time[index].pow(2) - 4 * distance[index];
            let relative = (delta as f64).sqrt();

            let t1 = ((time[index] as f64 - relative) / 2.0).floor() as u64;
            let t2 = ((time[index] as f64 + relative) / 2.0).ceil() as u64;

            product *= t2 - t1 - 1;
        }

        product.to_string()
    }

    fn part2(&self, input: String) -> String {
        let lines = input.lines().collect::<Vec<&str>>();

        let time = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .filter(|s| !s.starts_with(' ') && !s.is_empty())
            .collect::<String>()
            .trim()
            .parse::<u64>()
            .unwrap();

        let distance = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .filter(|s| !s.starts_with(' ') && !s.is_empty())
            .collect::<String>()
            .trim()
            .parse::<u64>()
            .unwrap();

        let delta = time.pow(2) - 4 * distance;
        let relative = (delta as f64).sqrt();

        let t1 = ((time as f64 - relative) / 2.0).floor() as u64;
        let t2 = ((time as f64 + relative) / 2.0).ceil() as u64;

        (t2 - t1 - 1).to_string()
    }
}
