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
            let time = time[index];
            let distance = distance[index];
            let mut sum = 0;

            for i in 1..time {
                let new_distance = i * (time - i);

                if new_distance > distance {
                    sum += 1;
                }
            }

            product *= sum;
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

        let mut sum: u64 = 0;

        for i in 1..time {
            let new_distance = i * (time - i);

            if new_distance > distance {
                sum += 1;
            }
        }

        sum.to_string()
    }
}
