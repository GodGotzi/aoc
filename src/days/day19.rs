use std::collections::HashMap;

use itertools::Itertools;

use crate::api::Solution;

#[derive(Debug)]
struct Condition {
    right: u32,
    result: String,
    symbol: char,
    var: char,
}

impl Condition {
    fn new(right: u32, result: String, symbol: char, var: char) -> Self {
        Self {
            right,
            result,
            symbol,
            var,
        }
    }

    fn check(&self, value: u32) -> Option<&str> {
        match self.symbol {
            '<' => {
                if value < self.right {
                    Some(&self.result)
                } else {
                    None
                }
            }
            '>' => {
                if value > self.right {
                    Some(&self.result)
                } else {
                    None
                }
            }
            _ => panic!("Unknown symbol: {}", self.symbol),
        }
    }
}

pub struct Day19;

impl Solution for Day19 {
    fn get_day(&self) -> u8 {
        19
    }

    fn part1(&self, input: String) -> String {
        let (workflows, ratings) = input.split_once("\n\n").unwrap();

        let workflows = parse_workflows(workflows);
        let ratings = parse_ratings(ratings);

        let mut rating_sum = 0;

        for rating in ratings {
            let mut current_workflow = "in";

            while current_workflow != "A" && current_workflow != "R" {
                let (conditions, right) = workflows.get(current_workflow).unwrap();

                let mut result = None;

                for condition in conditions {
                    if let Some(new_result) = condition.check(*rating.get(&condition.var).unwrap())
                    {
                        result = Some(new_result);
                        break;
                    }
                }

                if result.is_none() {
                    current_workflow = right;
                } else {
                    current_workflow = result.unwrap();
                }
            }

            if current_workflow == "A" {
                rating_sum += rating.values().copied().sum::<u32>();
            }
        }

        rating_sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        "".to_string()
    }
}

fn parse_workflows(workflow: &str) -> HashMap<String, (Vec<Condition>, String)> {
    let mut workflows = HashMap::new();

    for raw_workflow in workflow.lines() {
        let (name, conditions) = raw_workflow.split_once('{').unwrap();
        let conditions = conditions.replace('}', "");
        let mut conditions = conditions.split(',').collect_vec();
        let right = conditions.remove(conditions.len() - 1);

        let conditions = conditions
            .iter()
            .map(|raw_condition| {
                let (condition, result) = raw_condition.split_once(':').unwrap();

                let symbol = if condition.contains('<') { '<' } else { '>' };

                let (var, value) = condition.split_once(['<', '>']).unwrap();

                Condition::new(
                    value.parse::<u32>().unwrap(),
                    result.to_string(),
                    symbol,
                    var.chars().next().unwrap(),
                )
            })
            .collect_vec();

        workflows.insert(name.to_string(), (conditions, right.to_string()));
    }

    workflows
}

fn parse_ratings(ratings: &str) -> Vec<HashMap<char, u32>> {
    ratings
        .lines()
        .map(|raw_workflow| {
            let map = raw_workflow.replace(['{', '}'], "").split(',').fold(
                HashMap::new(),
                |mut map, rating| {
                    let (value, rating) = rating.split_once('=').unwrap();
                    map.insert(
                        value.chars().next().unwrap(),
                        rating.parse::<u32>().unwrap(),
                    );

                    map
                },
            );

            map
        })
        .collect_vec()
}
