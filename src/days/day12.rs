use std::collections::HashMap;

use itertools::Itertools;

use crate::api::{IndexString, Solution};

pub struct Day12;

impl Solution for Day12 {
    fn get_day(&self) -> u8 {
        12
    }

    fn part1(&self, input: String) -> String {
        let mut sum: usize = 0;

        let mut cache: HashMap<(String, String), usize> = HashMap::new();

        for (spring, values) in input.lines().map(|line| {
            line.split_once(' ')
                .map(|(springs, values)| (springs, values))
                .unwrap()
        }) {
            let groups = values
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();

            let arrangements = check(spring, &groups, &groups, "".to_string(), &mut cache);

            sum += arrangements;
        }

        sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut sum: usize = 0;

        let mut cache: HashMap<(String, String), usize> = HashMap::new();

        for (mut springs, values) in input.lines().map(|line| {
            line.split_once(' ')
                .map(|(springs, values)| (springs.chars().collect_vec(), values))
                .unwrap()
        }) {
            let groups = values
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
                .repeat(5);

            let n = springs.len();
            springs.push('?');
            springs.extend_from_within(..);
            springs.extend_from_within(..);
            springs.extend_from_within(..n);

            let springs = springs.iter().join("");

            let arrangements = check(&springs, &groups, &groups, "".to_string(), &mut cache);

            sum += arrangements;
        }

        sum.to_string()
    }
}

fn check(
    spring: &str,
    current_groups: &[usize],
    all_groups: &[usize],
    mut buffer: String,
    cache: &mut HashMap<(String, String), usize>,
) -> usize {
    if let Some(value) = cache.get(&(spring.to_string(), current_groups.iter().join(","))) {
        return *value;
    }

    if current_groups.is_empty() || spring.is_empty() {
        return if is_valid(&buffer, all_groups) && !spring.contains('#') {
            1
        } else {
            0
        };
    }

    let first = spring.chars().next().unwrap();

    if first == '.' {
        buffer.push('.');

        let ret = check(&spring[1..], current_groups, all_groups, buffer, cache);

        cache.insert(
            (spring[1..].to_string(), current_groups.iter().join(",")),
            ret,
        );

        ret
    } else if first == '?' {
        let mut str = spring.to_string();
        str.replace_range(0..1, ".");

        let working = check(&str, current_groups, all_groups, buffer.clone(), cache);

        str.replace_range(0..1, "#");
        let broken = check(&str, current_groups, all_groups, buffer.clone(), cache);

        return working + broken;
    } else if get_range_length(spring, &['#', '?']) >= current_groups[0]
        && get_range_length(spring, &['#']) <= current_groups[0]
    {
        buffer.push_str("#".repeat(current_groups[0]).as_str());

        let ret = check(
            &spring[current_groups[0]..],
            &current_groups[1..],
            all_groups,
            buffer.clone(),
            cache,
        );

        cache.insert(
            (
                spring[current_groups[0]..].to_string(),
                current_groups[1..].iter().join(","),
            ),
            ret,
        );

        ret
    } else {
        return 0;
    }
}

fn get_range_length(spring: &str, charackters: &[char]) -> usize {
    let mut length = 0;

    for c in spring.chars() {
        if charackters.contains(&c) {
            length += 1;
        } else {
            break;
        }
    }

    length
}

fn is_valid(spring: &str, groups: &[usize]) -> bool {
    let mut temp = spring.to_string();

    for amount in groups.iter() {
        if let Some((offset, length)) = find_first_range(&temp, &['#']) {
            if *amount != length {
                return false;
            }

            temp.replace_range(offset..offset + length, "X".repeat(length).as_str());
        } else {
            return false;
        }
    }

    true
}

fn find_first_range(spring: &str, charackters: &[char]) -> Option<(usize, usize)> {
    if let Some(offset) = spring.find(charackters) {
        let mut length: usize = 0;

        for i in offset..spring.len() {
            if charackters.contains(&spring.index(i)) {
                length += 1;
            } else {
                break;
            }
        }

        return Some((offset, length));
    };

    None
}

mod test {

    #[test]
    pub fn test_is_valid() {
        assert!(super::is_valid("###...#..####", &[3, 1, 4]));
        assert!(super::is_valid("##....#..##.#.", &[2, 1, 2, 1]));

        assert!(super::is_valid("###...#..###", &[3, 1, 3]));
    }
}
