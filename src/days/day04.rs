use std::collections::HashMap;

use crate::api::Solution;

pub struct Day04;

impl Solution for Day04 {
    fn get_day(&self) -> u8 {
        4
    }

    fn part1(&self, input: String) -> String {
        let mut sum = 0;

        for line in input.lines() {
            let (_card, numbers) = line.split_once(": ").unwrap();
            let (guess_numbers, numbers) = numbers.split_once(" | ").unwrap();

            let guess_numbers = guess_numbers
                .split(' ')
                .filter(|num| !num.is_empty())
                .map(|num| num.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let numbers = numbers
                .split(' ')
                .filter(|num| !num.is_empty())
                .map(|num| num.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            sum += 2_u32.pow(
                guess_numbers
                    .iter()
                    .filter(|num| numbers.contains(num))
                    .count() as u32
                    - 1,
            );
        }

        sum.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut cards: HashMap<usize, (usize, Vec<u32>, Vec<u32>)> = HashMap::new();

        for line in input.lines() {
            let (card, numbers) = line.split_once(": ").unwrap();
            let (guess_numbers, numbers) = numbers.split_once(" | ").unwrap();

            let card_splitted = card.split(' ').collect::<Vec<&str>>();
            let card_id = card_splitted[card_splitted.len() - 1]
                .parse::<usize>()
                .unwrap();

            let guess_numbers = guess_numbers
                .split(' ')
                .filter(|num| !num.is_empty())
                .map(|num| num.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let numbers = numbers
                .split(' ')
                .filter(|num| !num.is_empty())
                .map(|num| num.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if let Some(value) = cards.get_mut(&card_id) {
                value.0 += 1;
            } else {
                cards.insert(card_id - 1, (1, guess_numbers, numbers));
            }
        }

        let mut id = 0;

        while id < cards.len() {
            handle_card(&mut cards, id);

            id += 1;
        }

        cards
            .values()
            .map(|value| value.0)
            .sum::<usize>()
            .to_string()
    }
}

fn handle_card(cards: &mut HashMap<usize, (usize, Vec<u32>, Vec<u32>)>, id: usize) {
    let (amount, guess_numbers, numbers) = cards.get(&id).unwrap().clone();

    for _ in 0..amount {
        let matches = guess_numbers
            .iter()
            .filter(|num| numbers.contains(num))
            .count();

        for id in id + 1..=id + matches {
            cards.get_mut(&id).unwrap().0 += 1;
        }
    }
}
