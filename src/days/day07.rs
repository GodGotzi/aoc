use phf::phf_map;

use crate::api::Solution;

static VALENCY: phf::Map<char, usize> = phf_map! {
    '2' => 1,
    '3' => 2,
    '4' => 3,
    '5' => 4,
    '6' => 5,
    '7' => 6,
    '8' => 7,
    '9' => 8,
    'T' => 9,
    'J' => 10,
    'Q' => 11,
    'K' => 12,
    'A' => 13,
};

enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

pub struct Day07;

impl Solution for Day07 {
    fn get_day(&self) -> u8 {
        7
    }

    fn part1(&self, input: String) -> String {
        let mut hands: Vec<(&str, usize)> = input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let bid = bid.parse::<usize>().unwrap();

                (hand, bid)
            })
            .collect();

        hands.sort_by(|(hand_a, _bid_a), (hand_b, _bid_b)| {
            let kind_a = get_kind(&get_counts(hand_a));
            let kind_b = get_kind(&get_counts(hand_b));

            let mut order = (kind_a as usize).cmp(&(kind_b as usize));

            for i in 0..5 {
                order = order.then(
                    get_valency(hand_a.chars().nth(i).unwrap())
                        .cmp(&get_valency(hand_b.chars().nth(i).unwrap())),
                );
            }

            order
        });

        calculate_hands(hands)
    }

    fn part2(&self, input: String) -> String {
        let mut hands: Vec<(&str, usize)> = input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let bid = bid.parse::<usize>().unwrap();

                (hand, bid)
            })
            .collect();

        hands.sort_by(|(hand_a, _bid_a), (hand_b, _bid_b)| {
            let kind_a = get_joker_kind(hand_a);
            let kind_b = get_joker_kind(hand_b);

            let mut order = (kind_a as usize).cmp(&(kind_b as usize));

            for i in 0..5 {
                order = order.then(
                    get_joker_valency(hand_a.chars().nth(i).unwrap())
                        .cmp(&get_joker_valency(hand_b.chars().nth(i).unwrap())),
                );
            }

            order
        });

        calculate_hands(hands)
    }
}

fn get_joker_kind(hand: &str) -> Kind {
    let mut counts = get_counts(&hand.replace('J', ""));
    counts[0] += hand.chars().filter(|card| *card == 'J').count();

    get_kind(&counts)
}

fn get_kind(counts: &[usize]) -> Kind {
    match counts[0] {
        5 => Kind::Five,
        4 => Kind::Four,
        3 => {
            if counts.len() == 2 {
                Kind::FullHouse
            } else {
                Kind::Three
            }
        }
        2 => {
            if counts.len() == 3 {
                Kind::TwoPair
            } else {
                Kind::OnePair
            }
        }
        1 => Kind::HighCard,
        _ => panic!("Invalid count: {}", counts[counts.len() - 1]),
    }
}

fn get_counts(hand: &str) -> Vec<usize> {
    let mut counts = hand.chars().fold(vec![0; 13], |mut count, card| {
        *count.get_mut(get_valency(card) - 1).unwrap() += 1;
        count
    });
    counts.sort();
    counts.reverse();

    counts
}

fn calculate_hands(hands: Vec<(&str, usize)>) -> String {
    hands
        .iter()
        .enumerate()
        .map(|(index, (_vec, bid))| (index + 1) * (*bid))
        .sum::<usize>()
        .to_string()
}

fn get_valency(card: char) -> usize {
    *VALENCY.get(&card).unwrap()
}

fn get_joker_valency(card: char) -> usize {
    if card == 'J' {
        0
    } else {
        get_valency(card)
    }
}
