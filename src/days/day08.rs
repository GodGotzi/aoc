use std::collections::HashMap;

use itertools::Itertools;

use crate::api::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn get_day(&self) -> u8 {
        8
    }

    fn part1(&self, input: String) -> String {
        let lines = input.lines().collect::<Vec<&str>>();
        let instruction = lines[0].chars().collect::<Vec<char>>();

        let map = lines[2..lines.len()]
            .iter()
            .fold(HashMap::new(), |mut map, line| {
                let (key, raw_pair) = line.split_once(" = ").unwrap();
                let (left, right) = raw_pair[1..raw_pair.len() - 1].split_once(", ").unwrap();
                map.insert(key.to_string(), (left.to_string(), right.to_string()));
                map
            });

        calculate_steps(&map, &instruction, "AAA", &|end| end == "ZZZ").to_string()
    }

    fn part2(&self, input: String) -> String {
        let lines = input.lines().collect::<Vec<&str>>();
        let instruction = lines[0].chars().collect::<Vec<char>>();

        let map = lines[2..lines.len()]
            .iter()
            .fold(HashMap::new(), |mut map, line| {
                let (key, raw_pair) = line.split_once(" = ").unwrap();
                let (left, right) = raw_pair[1..raw_pair.len() - 1].split_once(", ").unwrap();
                map.insert(key.to_string(), (left.to_string(), right.to_string()));
                map
            });

        let positions = map
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|key| key.to_string())
            .collect::<Vec<String>>();

        let mut current: Option<Vec<usize>> = None;

        for start in positions {
            let steps = calculate_steps(&map, &instruction, &start, &|end| end.ends_with('Z'));
            let prime_factors = to_prime_factors(steps);

            current = Some(least_multiple(
                &prime_factors,
                current.as_ref().unwrap_or(&prime_factors),
            ));
        }

        let mut product: usize = 1;

        for number in current.unwrap() {
            product *= number;
        }

        product.to_string()
    }
}

fn calculate_steps(
    map: &HashMap<String, (String, String)>,
    instruction: &Vec<char>,
    start: &str,
    is_end: &dyn Fn(&str) -> bool,
) -> usize {
    let mut current_position = start.to_string();

    let mut instruction_index = 0;
    let mut step_counter: usize = 0;

    while !is_end(&current_position) {
        let (left, right) = map.get(&current_position).unwrap();

        current_position = match instruction[instruction_index] {
            'L' => left.clone(),
            'R' => right.clone(),
            _ => panic!("Unknown instruction"),
        };

        instruction_index += 1;

        if instruction_index >= instruction.len() {
            instruction_index = 0;
        }

        step_counter += 1;
    }

    step_counter
}

fn to_prime_factors(mut number: usize) -> Vec<usize> {
    let mut factors = Vec::new();

    while number % 2 == 0 {
        factors.push(2);
        number /= 2;
    }

    let mut i = 3;

    while i * i <= number {
        while number % i == 0 {
            factors.push(i);
            number /= i;
        }

        i += 2;
    }

    if number > 2 {
        factors.push(number);
    }

    factors
}

fn least_multiple(num: &[usize], current: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut all = num.to_vec();
    all.extend(current.to_vec());

    for number in all.iter().unique() {
        let occurrences = num.iter().filter(|&n| *n == *number).count();
        let current_occurrences = current.iter().filter(|&n| *n == *number).count();

        for _ in 0..occurrences.max(current_occurrences) {
            result.push(*number);
        }
    }

    result
}

mod test {

    #[test]
    fn test_to_prime() {
        assert_eq!(super::to_prime_factors(2), vec![2]);
        assert_eq!(super::to_prime_factors(4), vec![2, 2]);
        assert_eq!(super::to_prime_factors(6), vec![2, 3]);
        assert_eq!(super::to_prime_factors(8), vec![2, 2, 2]);
        assert_eq!(super::to_prime_factors(9), vec![3, 3]);
        assert_eq!(super::to_prime_factors(12), vec![2, 2, 3]);
        assert_eq!(super::to_prime_factors(15), vec![3, 5]);
        assert_eq!(super::to_prime_factors(21), vec![3, 7]);
        assert_eq!(super::to_prime_factors(24), vec![2, 2, 2, 3]);
        assert_eq!(super::to_prime_factors(30), vec![2, 3, 5]);
    }

    #[test]
    fn test_least_multiple() {
        assert_eq!(super::least_multiple(&[2], &[2]), vec![2]);
        assert_eq!(super::least_multiple(&[2], &[3]), vec![2, 3]);
        assert_eq!(super::least_multiple(&[2, 2], &[2]), vec![2, 2]);
    }
}
