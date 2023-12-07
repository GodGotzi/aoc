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

static PART2_VALENCY: phf::Map<char, usize> = phf_map! {
    'J' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'T' => 10,
    'Q' => 11,
    'K' => 12,
    'A' => 13,
};

#[derive(Debug)]
enum Kind {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
    OnePair,
    HighCard,
}

pub struct Day07;

impl Solution for Day07 {
    fn get_day(&self) -> u8 {
        7
    }

    fn part1(&self, input: String) -> String {
        let mut hands: Vec<(Vec<char>, usize)> = input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let bid = bid.parse::<usize>().unwrap();
                let hand = hand.chars().collect::<Vec<char>>();

                (hand, bid)
            })
            .collect();

        hands.sort_by(|(hand_a, _bid_a), (hand_b, _bid_b)| {
            let kind_a = get_kind_part1(hand_a);
            let kind_b = get_kind_part1(hand_b);

            (kind_b as usize)
                .cmp(&(kind_a as usize))
                .then(
                    VALENCY
                        .get(&hand_a[0])
                        .unwrap()
                        .cmp(VALENCY.get(&hand_b[0]).unwrap()),
                )
                .then(
                    VALENCY
                        .get(&hand_a[1])
                        .unwrap()
                        .cmp(VALENCY.get(&hand_b[1]).unwrap()),
                )
                .then(
                    VALENCY
                        .get(&hand_a[2])
                        .unwrap()
                        .cmp(VALENCY.get(&hand_b[2]).unwrap()),
                )
                .then(
                    VALENCY
                        .get(&hand_a[3])
                        .unwrap()
                        .cmp(VALENCY.get(&hand_b[3]).unwrap()),
                )
                .then(
                    VALENCY
                        .get(&hand_a[4])
                        .unwrap()
                        .cmp(VALENCY.get(&hand_b[4]).unwrap()),
                )
        });

        hands
            .iter()
            .enumerate()
            .map(|(index, (_vec, bid))| {
                println!("{} {}", hands.len() - index, bid);
                (index + 1) * (*bid)
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut hands: Vec<(Vec<char>, usize)> = input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let bid = bid.parse::<usize>().unwrap();
                let hand = hand.chars().collect::<Vec<char>>();

                (hand, bid)
            })
            .collect();

        hands.sort_by(|(hand_a, _bid_a), (hand_b, _bid_b)| {
            let kind_a = get_kind_part2(hand_a);
            let kind_b = get_kind_part2(hand_b);

            (kind_b as usize)
                .cmp(&(kind_a as usize))
                .then(
                    PART2_VALENCY
                        .get(&hand_a[0])
                        .unwrap()
                        .cmp(PART2_VALENCY.get(&hand_b[0]).unwrap()),
                )
                .then(
                    PART2_VALENCY
                        .get(&hand_a[1])
                        .unwrap()
                        .cmp(PART2_VALENCY.get(&hand_b[1]).unwrap()),
                )
                .then(
                    PART2_VALENCY
                        .get(&hand_a[2])
                        .unwrap()
                        .cmp(PART2_VALENCY.get(&hand_b[2]).unwrap()),
                )
                .then(
                    PART2_VALENCY
                        .get(&hand_a[3])
                        .unwrap()
                        .cmp(PART2_VALENCY.get(&hand_b[3]).unwrap()),
                )
                .then(
                    PART2_VALENCY
                        .get(&hand_a[4])
                        .unwrap()
                        .cmp(PART2_VALENCY.get(&hand_b[4]).unwrap()),
                )
        });

        hands
            .iter()
            .enumerate()
            .map(|(index, (_vec, bid))| {
                println!("{} {}", hands.len() - index, bid);
                (index + 1) * (*bid)
            })
            .sum::<usize>()
            .to_string()
    }
}

fn get_kind_part1(hand: &[char]) -> Kind {
    let valencies: Vec<usize> = hand.iter().map(|card| VALENCY[card]).collect();

    let mut counts: Vec<usize> = vec![0; 13];
    for valency in valencies {
        counts[valency - 1] += 1;
    }

    let mut counts: Vec<(usize, usize)> = counts
        .iter()
        .enumerate()
        .filter(|(_i, count)| **count > 0)
        .map(|(i, count)| (i, *count))
        .collect();
    counts.sort_by(|(_i_a, count_a), (_i_b, count_b)| count_b.cmp(count_a));

    let (_i, count) = counts[0];
    match count {
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
                Kind::TwoPairs
            } else {
                Kind::OnePair
            }
        }
        1 => Kind::HighCard,
        _ => panic!("Invalid count: {}", count),
    }
}

fn get_kind_part2(hand: &[char]) -> Kind {
    let valencies: Vec<usize> = hand.iter().map(|card| PART2_VALENCY[card]).collect();

    let mut counts: Vec<usize> = vec![0; 13];
    for valency in valencies {
        counts[valency - 1] += 1;
    }

    let joker_count = counts[0];

    let mut counts: Vec<(usize, usize)> = counts
        .iter()
        .enumerate()
        .filter(|(i, count)| **count > 0 && *i != 0)
        .map(|(i, count)| (i, *count))
        .collect();

    counts.sort_by(|(_i_a, count_a), (_i_b, count_b)| count_b.cmp(count_a));

    if counts.is_empty() {
        return Kind::Five;
    }

    counts[0].1 += joker_count;

    let (_i, count) = counts[0];
    match count {
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
                Kind::TwoPairs
            } else {
                Kind::OnePair
            }
        }
        1 => Kind::HighCard,
        _ => panic!("Invalid count: {}", count),
    }
}
