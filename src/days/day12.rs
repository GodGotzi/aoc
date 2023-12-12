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

        for (spring, values) in input.lines().map(|line| {
            line.split_once(' ')
                .map(|(springs, values)| (springs.chars().collect_vec(), values))
                .unwrap()
        }) {
            let groups = values
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();

            let arrangements = check(spring.clone(), &groups, "".to_string(), 0, 0);

            sum += arrangements;
        }

        sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut sum: usize = 0;

        let cache: HashMap<String, usize> = HashMap::new();

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

            let arrangements = check(springs.clone(), &groups, "".to_string(), 0, 0);

            sum += arrangements;
        }

        sum.to_string()
    }
}

fn check(
    mut spring: Vec<char>,
    groups: &[usize],
    mut buffer: String,
    pos: usize,
    group: usize,
) -> usize {
    if group == groups.len() || pos >= spring.len() {
        return if is_valid(&buffer, groups) && !spring[pos..].contains(&'#') {
            1
        } else {
            0
        };
    }

    if spring[pos] == '.' {
        buffer.push('.');

        check(spring, groups, buffer, pos + 1, group)
    } else if spring[pos] == '?' {
        spring[pos] = '#';
        let check1 = check(spring.clone(), groups, buffer.clone(), pos, group);

        spring[pos] = '.';
        let check2 = check(spring.clone(), groups, buffer.clone(), pos, group);

        return check1 + check2;
    } else if get_range_length(&spring, pos, &['#', '?']) >= groups[group]
        && get_range_length(&spring, pos, &['#']) <= groups[group]
    {
        buffer.push_str("#".repeat(groups[group]).as_str());

        return check(spring, groups, buffer, pos + groups[group], group + 1);
    } else {
        return 0;
    }
}

fn get_range_length(spring: &[char], pos: usize, charackters: &[char]) -> usize {
    let mut length = 0;

    for c in spring[pos..].iter() {
        if charackters.contains(c) {
            length += 1;
        } else {
            break;
        }
    }

    length
}

fn is_valid(spring: &str, amounts: &[usize]) -> bool {
    let mut temp = spring.to_string();

    for amount in amounts.iter() {
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
