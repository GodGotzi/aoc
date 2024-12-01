use phf::phf_map;

use crate::api::{IVec2, Matrix2D, Solution};

static PIPE_CONNECTIONS: phf::Map<char, [[i32; 2]; 2]> = phf_map! {
    '|' => [[1, 0], [-1, 0]],
    '-' => [[0, 1], [0, -1]],
    'L' => [[-1, 0], [0, 1]],
    'J' => [[-1, 0], [0, -1]],
    '7' => [[1, 0], [0, -1]],
    'F' => [[1, 0], [0, 1]]
};

pub struct Day10;

impl Solution for Day10 {
    fn get_day(&self) -> u8 {
        10
    }

    fn part1(&self, input: String) -> String {
        let mut grid: Matrix2D<char> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
            .into();

        let largest = get_largest_loop(&mut grid);

        (largest.0 / 2).to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut grid: Matrix2D<char> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
            .into();

        let (_loop_size, visited) = get_largest_loop(&mut grid);

        get_enclosed(&grid, &visited).to_string()
    }
}

fn get_enclosed(grid: &Matrix2D<char>, visited: &Matrix2D<bool>) -> usize {
    let mut counter = 0;

    grid.iter_enumerate().for_each(|(pos, _val)| {
        if !visited.get(&(pos.0 as i32, pos.1 as i32)).unwrap_or(&true) {
            let mut intersect_counter = 0;
            let mut last = None;

            for index in pos.1 + 1..grid.size().1 {
                let value = grid.get(&(pos.0 as i32, index as i32)).unwrap();

                if *value == '7' {
                    if let Some(last) = last {
                        if last == 'L' {
                            intersect_counter += 1;
                        }
                    }
                } else if *value == 'J' {
                    if let Some(last) = last {
                        if last == 'F' {
                            intersect_counter += 1;
                        }
                    }
                } else if *value == 'L' {
                    last = Some('L');
                } else if *value == 'F' {
                    last = Some('F');
                } else if *value == '|' {
                    intersect_counter += 1;
                }
            }

            if intersect_counter % 2 == 1 {
                counter += 1;
            }
        }
    });

    counter
}

fn get_largest_loop(grid: &mut Matrix2D<char>) -> (usize, Matrix2D<bool>) {
    let start = grid
        .iter_enumerate()
        .find(|(_, value)| **value == 'S')
        .unwrap()
        .0;

    grid.set(
        &(start.0 as i32, start.1 as i32),
        get_start_replacement(grid, (start.0 as i32, start.1 as i32)),
    );

    let mut queue: Vec<(usize, IVec2, Matrix2D<bool>)> = vec![(
        0,
        (start.0 as i32, start.1 as i32),
        Matrix2D::new(false, grid.size()),
    )];

    let mut largest: Option<(usize, Matrix2D<bool>)> = None;

    while let Some((counter, pos1, mut visited)) = queue.pop() {
        visited.set(&pos1, true);

        if start == (pos1.0 as usize, pos1.1 as usize)
            && counter > 0
            && (largest.is_none() || counter > largest.as_ref().unwrap().0)
        {
            largest = Some((counter, visited.clone()));
        }

        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        for direction in directions {
            let pos2 = (pos1.0 + direction.0, pos1.1 + direction.1);

            if (!*visited.get(&pos2).unwrap_or(&true) && is_connected(grid, pos1, direction))
                || start == (pos2.0 as usize, pos2.1 as usize)
            {
                queue.push((counter + 1, pos2, visited.clone()));
            }
        }
    }

    for (pos, value) in largest.as_ref().unwrap().1.iter_enumerate() {
        if !*value {
            grid.set(&(pos.0 as i32, pos.1 as i32), '.');
        }
    }

    largest.unwrap()
}

fn get_start_replacement(grid: &Matrix2D<char>, (x, y): (i32, i32)) -> char {
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut start_connections = Vec::new();

    for direction in directions {
        let pos = (x + direction.0, y + direction.1);

        if let Some(value) = grid.get(&pos) {
            if let Some(connections) = PIPE_CONNECTIONS.get(value) {
                if connections[0] == [-direction.0, -direction.1] {
                    start_connections.push(direction);
                }

                if connections[1] == [-direction.0, -direction.1] {
                    start_connections.push(direction);
                }
            }
        }
    }

    *PIPE_CONNECTIONS
        .entries()
        .find(|(_, connections)| {
            (connections[0] == [start_connections[0].0, start_connections[0].1]
                && connections[1] == [start_connections[1].0, start_connections[1].1])
                || (connections[1] == [start_connections[0].0, start_connections[0].1]
                    && connections[0] == [start_connections[1].0, start_connections[1].1])
        })
        .unwrap()
        .0
}

fn is_connected(
    grid: &Matrix2D<char>,
    (pos_x, pos_y): (i32, i32),
    (dir_x, dir_y): (i32, i32),
) -> bool {
    let current = *grid.get(&(pos_x, pos_y)).unwrap();
    let other = grid.get(&(pos_x + dir_x, pos_y + dir_y));

    if other.is_none() || other.unwrap() == &'.' {
        return false;
    }

    let connections = PIPE_CONNECTIONS.get(&current).unwrap();
    let other_connections = PIPE_CONNECTIONS.get(other.unwrap()).unwrap();

    if (connections[0] == [dir_x, dir_y] || connections[1] == [dir_x, dir_y])
        && (other_connections[0] == [-dir_x, -dir_y] || other_connections[1] == [-dir_x, -dir_y])
    {
        return true;
    }

    false
}
