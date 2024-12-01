use crate::api::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn get_day(&self) -> u8 {
        1
    }

    fn get_year(&self) -> u16 {
        2024
    }

    fn part1(&self, input: String) -> String {
        let mut left_locations = vec![];
        let mut right_locations = vec![];

        for line in input.lines() {
            let (left, right) = line.split_once("   ").unwrap();

            left_locations.push(left.parse::<i32>().unwrap());
            right_locations.push(right.parse::<i32>().unwrap());
        }

        left_locations.sort();
        right_locations.sort();

        let mut total_distance = 0;

        for i in 0..left_locations.len() {
            total_distance += (right_locations[i] - left_locations[i]).abs();
        }

        total_distance.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut left_locations = vec![];
        let mut right_locations = vec![];

        for line in input.lines() {
            let (left, right) = line.split_once("   ").unwrap();

            left_locations.push(left.parse::<u32>().unwrap());
            right_locations.push(right.parse::<u32>().unwrap());
        }

        let mut similarity_count = 0;

        for left in left_locations {
            similarity_count += left as usize
                * right_locations
                    .iter()
                    .filter(|&&right| left == right)
                    .count();
        }

        similarity_count.to_string()
    }
}
