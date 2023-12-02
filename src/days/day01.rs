use std::str::FromStr;

use phf::phf_map;

use crate::api::Solution;

pub struct Day01;

static STR_NUMBERS: phf::Map<&'static str, &'static str> = phf_map! {
    "ONE" => "O1E",
    "TWO" => "T2W",
    "THREE" => "T3E",
    "FOUR" => "F4R",
    "FIVE" => "F5E",
    "SIX" => "S6X",
    "SEVEN" => "S7N",
    "EIGHT" => "E8T",
    "NINE" => "N9E",
};

impl Solution for Day01 {
    fn get_day(&self) -> u8 {
        1
    }

    fn part1(&self, input: String) -> String {
        let mut counter: u32 = 0;

        for line in input.lines() {
            let mut buffer = String::new();

            let numbers = line
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>();

            buffer.push(numbers[0]);
            buffer.push(*numbers.last().unwrap());

            counter += buffer.parse::<u32>().unwrap();
        }

        counter.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut counter: u32 = 0;

        for raw_line in input.lines() {
            let mut buffer = String::new();
            let line = replace_str_number(raw_line.to_uppercase().as_str());

            let numbers = line
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>();

            buffer.push(numbers[0]);
            buffer.push(*numbers.last().unwrap());

            counter += buffer.parse::<u32>().unwrap();
        }

        counter.to_string()
    }
}

fn replace_str_number(string: &str) -> String {
    let mut buffer = String::from_str(string).unwrap();

    for (key, value) in STR_NUMBERS.entries() {
        buffer = buffer.replace(*key, value);
    }

    buffer
}
