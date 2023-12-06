use phf::phf_map;

use crate::api::Solution;

pub struct Day01;

static PART1_STR_NUMBERS: phf::Map<&'static str, char> = phf_map! {
    "1" => '1',
    "2" => '2',
    "3" => '3',
    "4" => '4',
    "5" => '5',
    "6" => '6',
    "7" => '7',
    "8" => '8',
    "9" => '9',
};

static PART2_STR_NUMBERS: phf::Map<&'static str, char> = phf_map! {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
    "1" => '1',
    "2" => '2',
    "3" => '3',
    "4" => '4',
    "5" => '5',
    "6" => '6',
    "7" => '7',
    "8" => '8',
    "9" => '9',
};

impl Solution for Day01 {
    fn get_day(&self) -> u8 {
        1
    }

    fn part1(&self, input: String) -> String {
        compute(input, &PART1_STR_NUMBERS).to_string()
    }

    fn part2(&self, input: String) -> String {
        compute(input, &PART2_STR_NUMBERS).to_string()
    }
}

fn compute(input: String, map: &phf::Map<&'static str, char>) -> u32 {
    let mut counter: u32 = 0;

    for raw_line in input.lines() {
        let mut first = None;
        let mut last = None;

        let mut temp = raw_line.to_string();

        'outer: while !temp.is_empty() {
            for (raw, value) in map.entries() {
                if temp.starts_with(raw) {
                    first = Some(value);
                    break 'outer;
                }
            }

            temp.replace_range(0..1, "");
        }

        temp = raw_line.to_string();

        'outer: while !temp.is_empty() {
            for (raw, value) in map.entries() {
                if temp.ends_with(raw) {
                    last = Some(value);
                    break 'outer;
                }
            }

            temp.replace_range(temp.len() - 1..temp.len(), "");
        }

        let mut buffer = String::new();

        buffer.push(*first.unwrap());
        buffer.push(*last.unwrap());

        counter += buffer.parse::<u32>().unwrap();
    }

    counter
}
