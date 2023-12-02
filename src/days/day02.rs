use crate::api::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn get_day(&self) -> u8 {
        2
    }

    fn part1(&self, input: String) -> String {
        static RED_MAX: u32 = 12;
        static GREEN_MAX: u32 = 13;
        static BLUE_MAX: u32 = 14;

        let mut game_counter = 0;

        for line in input.lines() {
            let (game, sets) = line.split_once(':').unwrap();
            let game_id = game.split_once(' ').unwrap().1.parse::<u32>();

            let mut valid = true;
            let cubes = sets.split([',', ';']).collect::<Vec<&str>>();

            for cube in cubes {
                let (amount, cube_type) = cube[1..cube.len()].split_once(' ').unwrap();
                let amount = amount.parse::<u32>().unwrap();

                if match cube_type {
                    "red" => amount > RED_MAX,
                    "green" => amount > GREEN_MAX,
                    "blue" => amount > BLUE_MAX,
                    _ => panic!("Unknown cube type: {}", cube_type),
                } {
                    valid = false;
                }
            }

            if valid {
                game_counter += game_id.unwrap();
            }
        }

        game_counter.to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut power_sum: u32 = 0;

        for line in input.lines() {
            let (_game, sets) = line.split_once(':').unwrap();

            let mut max = (0, 0, 0);
            let cubes = sets.split([',', ';']).collect::<Vec<&str>>();

            for cube in cubes {
                let (amount, cube_type) = cube[1..cube.len()].split_once(' ').unwrap();
                let amount = amount.parse::<u32>().unwrap();

                match cube_type {
                    "red" => {
                        if amount > max.0 {
                            max.0 = amount;
                        }
                    }
                    "green" => {
                        if amount > max.1 {
                            max.1 = amount;
                        }
                    }
                    "blue" => {
                        if amount > max.2 {
                            max.2 = amount;
                        }
                    }
                    _ => panic!("Unknown cube type: {}", cube_type),
                };
            }

            power_sum += max.0 * max.1 * max.2;
        }

        power_sum.to_string()
    }
}
