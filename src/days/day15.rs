use itertools::Itertools;

use crate::api::Solution;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Dash,
    Equals(u8),
}

pub struct Day15;

impl Solution for Day15 {
    fn get_day(&self) -> u8 {
        15
    }

    fn part1(&self, input: String) -> String {
        let first = input.lines().next().unwrap();
        let seq = first.split(',').collect_vec();

        seq.iter().map(|s| hash(s)).sum::<usize>().to_string()
    }

    fn part2(&self, input: String) -> String {
        let first = input.lines().next().unwrap();
        let seq = first.split(',').collect_vec();

        let seq = seq
            .iter()
            .map(|instruction| {
                if instruction.contains('=') {
                    let (instruction, value) = instruction.split_once('=').unwrap();
                    let value = value.parse::<u8>().unwrap();

                    (instruction, Operation::Equals(value))
                } else {
                    let instruction = instruction.split_once('-').unwrap().0;
                    (instruction, Operation::Dash)
                }
            })
            .collect_vec();

        let boxes = seq.iter().fold(
            vec![Vec::<(&str, u8)>::new(); 256],
            |mut boxes, (instruction, operation)| {
                let hash = hash(instruction);

                if operation == &Operation::Dash {
                    if let Some((index, _)) = boxes[hash]
                        .iter()
                        .enumerate()
                        .find(|(_, (inner, _))| inner == instruction)
                    {
                        boxes[hash].remove(index);
                    }
                } else if let Operation::Equals(lens) = *operation {
                    if let Some((index, _)) = boxes[hash]
                        .iter()
                        .enumerate()
                        .find(|(_, (inner, _))| inner == instruction)
                    {
                        boxes[hash].remove(index);
                        boxes[hash].insert(0, (instruction, lens))
                    } else {
                        boxes[hash].push((instruction, lens))
                    }
                }

                boxes
            },
        );

        let sum = boxes
            .iter()
            .enumerate()
            .map(|(box_index, b)| {
                b.iter()
                    .enumerate()
                    .map(|(index, (_, lens))| (box_index + 1) * (index + 1) * (*lens as usize))
                    .sum::<usize>()
            })
            .sum::<usize>();

        sum.to_string()
    }
}

fn hash(str: &str) -> usize {
    let mut hash = 0;

    for c in str.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}
