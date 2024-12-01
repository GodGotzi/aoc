use crate::api::Solution;

pub struct Day05;

impl Solution for Day05 {
    fn get_day(&self) -> u8 {
        5
    }

    fn part1(&self, input: String) -> String {
        let lines = input.lines().collect::<Vec<&str>>();
        let mut seeds = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let maps = lines[2..lines.len()]
            .split(|line| line.is_empty())
            .collect::<Vec<&[&str]>>();

        for map in maps {
            let mut seeds_changed: Vec<bool> = vec![false; seeds.len()];

            for mapping in map.iter().skip(1) {
                let values = mapping
                    .split(' ')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let (dest_start, source_start, range) = (values[0], values[1], values[2]);

                for (index, source) in seeds.iter_mut().enumerate() {
                    if *source >= source_start
                        && *source < source_start + range
                        && !seeds_changed[index]
                    {
                        *source = dest_start + (*source - source_start);

                        seeds_changed[index] = true;
                    }
                }
            }
        }

        seeds.sort();

        seeds[0].to_string()
    }

    fn part2(&self, input: String) -> String {
        let lines = input.lines().collect::<Vec<&str>>();
        let seeds_splitted = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut seeds: Vec<(usize, usize)> = seeds_splitted
            .chunks_exact(2)
            .map(|seeds| (seeds[0], seeds[1]))
            .collect::<Vec<(usize, usize)>>();

        let maps = lines[2..lines.len()]
            .split(|line| line.is_empty())
            .map(|map| map.iter().skip(1))
            .map(|iter| {
                iter.map(|str| {
                    str.split(' ')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
            })
            .collect::<Vec<Vec<Vec<usize>>>>();

        println!("ok");

        for map in maps {
            let mut updated_seeds: Vec<(usize, usize)> = vec![];

            while let Some((seed_start, seed_range)) = seeds.pop() {
                let mut found = false;

                for mapping in map.iter().skip(1) {
                    let (dest_start, source_start, range) = (mapping[0], mapping[1], mapping[2]);

                    if let Some((start, length)) =
                        overlap((seed_start, seed_range), (source_start, range))
                    {
                        updated_seeds.push((start + (dest_start - source_start), length));

                        if seed_start < source_start {
                            seeds.push((seed_start, source_start - seed_start));
                        }

                        if seed_start + seed_range > source_start + range {
                            seeds.push((
                                start + length,
                                seed_start + seed_range - source_start - range,
                            ));
                        }

                        found = true;
                    }
                }

                if !found {
                    updated_seeds.push((seed_start, seed_range));
                }
            }

            seeds = updated_seeds;
        }

        seeds.sort();

        seeds[0].0.to_string()
    }
}

fn overlap(seeds: (usize, usize), source: (usize, usize)) -> Option<(usize, usize)> {
    if seeds.0 < source.0 + source.1 && seeds.0 + seeds.1 > source.0 {
        let start = if seeds.0 > source.0 {
            seeds.0
        } else {
            source.0
        };

        let end = if seeds.0 + seeds.1 < source.0 + source.1 {
            seeds.0 + seeds.1
        } else {
            source.0 + source.1
        };

        Some((start, end - start))
    } else {
        None
    }
}
