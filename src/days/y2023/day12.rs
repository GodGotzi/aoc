use std::collections::HashMap;

use itertools::Itertools;

use crate::api::Solution;

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

            let arrangements = check(spring, &groups, &mut cache);

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

            let arrangements = check(&springs, &groups, &mut cache);

            sum += arrangements;
        }

        sum.to_string()
    }
}

fn check(spring: &str, groups: &[usize], cache: &mut HashMap<(String, String), usize>) -> usize {
    if let Some(value) = cache.get(&(spring.to_string(), groups.iter().join(","))) {
        return *value;
    }

    if groups.is_empty() {
        return if !spring.contains('#') { 1 } else { 0 };
    }

    if spring.is_empty() {
        return 0;
    }

    let first = spring.chars().next().unwrap();

    if first == '.' {
        let ret = check(&spring[1..], groups, cache);

        cache.insert((spring[1..].to_string(), groups.iter().join(",")), ret);

        ret
    } else if first == '?' {
        return check(&spring.replacen('?', ".", 1), groups, cache)
            + check(&spring.replacen('?', "#", 1), groups, cache);
    } else if first == '#' {
        if spring.len() < groups[0] {
            return 0;
        }

        if spring[..groups[0]].replace('?', "#") != "#".repeat(groups[0]) {
            return 0;
        }

        if spring.len() == groups[0] {
            if groups.len() == 1 {
                return 1;
            } else {
                return 0;
            }
        }

        if spring[groups[0]..].is_empty() && groups.len() == 1 {
            return 1;
        }

        if ['?', '.'].contains(&spring[groups[0]..].chars().next().unwrap()) {
            let ret = check(&spring[groups[0] + 1..], &groups[1..], cache);

            cache.insert(
                (
                    spring[groups[0] + 1..].to_string(),
                    groups[1..].iter().join(","),
                ),
                ret,
            );

            return ret;
        }

        return 0;
    } else {
        return 0;
    }
}
