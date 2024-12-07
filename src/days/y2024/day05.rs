use std::collections::HashMap;

use itertools::Itertools;

use crate::api::Solution;

#[derive(Debug)]
enum Order {
    Before(u32),
    After(u32),
}

pub struct Day05;

impl Solution for Day05 {
    fn get_day(&self) -> u8 {
        5
    }

    fn get_year(&self) -> u16 {
        2024
    }

    fn part1(&self, input: String) -> String {
        let (page_ordering_str, updates_str) = input.split_once("\n\n").unwrap();

        let page_ordering = page_ordering_str
            .lines()
            .map(|line| {
                let (page0, page1) = line.split_once("|").unwrap();
                (page0.parse::<u32>().unwrap(), page1.parse::<u32>().unwrap())
            })
            .collect::<Vec<(u32, u32)>>();

        let updates = updates_str
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let mut ordering_map = HashMap::new();

        for (page0, page1) in page_ordering {
            ordering_map
                .entry(page0)
                .or_insert(Vec::new())
                .push(Order::After(page1));

            ordering_map
                .entry(page1)
                .or_insert(Vec::new())
                .push(Order::Before(page0));
        }

        println!("{:?}", ordering_map);

        let correct_updates = updates
            .into_iter()
            .filter(|update| {
                let mut correct = true;

                for i in 0..update.len() {
                    let page = update[i];
                    let orderings = ordering_map.get(&page).unwrap();

                    for ordering in orderings {
                        match ordering {
                            Order::Before(before) => {
                                if update.contains(before) && !update[..i].contains(before) {
                                    correct = false;
                                    break;
                                }
                            }
                            Order::After(after) => {
                                if update.contains(after) && !update[i + 1..].contains(after) {
                                    correct = false;
                                    break;
                                }
                            }
                        }
                    }
                }

                correct
            })
            .collect_vec();

        let mut middle_sum = 0;

        for update in correct_updates {
            middle_sum += update[update.len() / 2];
        }

        middle_sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let (page_ordering_str, updates_str) = input.split_once("\n\n").unwrap();

        let page_ordering = page_ordering_str
            .lines()
            .map(|line| {
                let (page0, page1) = line.split_once("|").unwrap();
                (page0.parse::<u32>().unwrap(), page1.parse::<u32>().unwrap())
            })
            .collect::<Vec<(u32, u32)>>();

        let updates = updates_str
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let mut ordering_map = HashMap::new();

        for (page0, page1) in page_ordering {
            ordering_map
                .entry(page0)
                .or_insert(Vec::new())
                .push(Order::After(page1));

            ordering_map
                .entry(page1)
                .or_insert(Vec::new())
                .push(Order::Before(page0));
        }

        println!("{:?}", ordering_map);

        let incorrect_updates = updates
            .into_iter()
            .filter(|update| {
                let mut correct = true;

                for i in 0..update.len() {
                    let page = update[i];
                    let orderings = ordering_map.get(&page).unwrap();

                    for ordering in orderings {
                        match ordering {
                            Order::Before(before) => {
                                if update.contains(before) && !update[..i].contains(before) {
                                    correct = false;
                                    break;
                                }
                            }
                            Order::After(after) => {
                                if update.contains(after) && !update[i + 1..].contains(after) {
                                    correct = false;
                                    break;
                                }
                            }
                        }
                    }
                }

                !correct
            })
            .map(|update| {
                println!("{:?}", update);
                let mut new_update = update.clone();

                for _ in 0..update.len() {
                    for i in 0..update.len() {
                        let page = update[i];
                        let orderings = ordering_map.get(&page).unwrap();

                        for ordering in orderings {
                            match ordering {
                                Order::After(before) => {
                                    if update.contains(before) && !update[..i].contains(before) {
                                        new_update.swap(
                                            i,
                                            if update.iter().position(|&x| x == *before).unwrap()
                                                == update.len() - 1
                                            {
                                                update.len() - 1
                                            } else {
                                                update.iter().position(|&x| x == *before).unwrap()
                                                    + 1
                                            },
                                        );
                                    }
                                }
                                Order::Before(after) => {
                                    if update.contains(after) && !update[i + 1..].contains(after) {
                                        new_update.swap(
                                            i,
                                            if update.iter().position(|&x| x == *after).unwrap()
                                                == 0
                                            {
                                                0
                                            } else {
                                                update.iter().position(|&x| x == *after).unwrap()
                                                    - 1
                                            },
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                println!("{:?}", new_update);

                new_update
            })
            .collect_vec();

        let mut middle_sum = 0;

        for update in incorrect_updates {
            middle_sum += update[update.len() / 2];
        }

        middle_sum.to_string()
    }
}
