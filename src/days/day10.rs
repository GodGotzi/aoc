use phf::phf_map;

use crate::api::Solution;

static PIPE_CONNECTIONS: phf::Map<char, [[i32; 2]; 2]> = phf_map! {
    '|' => [[0, 1], [0, -1]],
    '-' => [[1, 0], [-1, 0]],
    'L' => [[0, -1], [1, 0]],
    'J' => [[0, -1], [-1, 0]],
    '7' => [[0, 1], [-1, 0]],
    'F' => [[0, 1], [1, 0]]
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn row(&self) -> i32 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }

    fn col(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }
}

pub struct Day10;

impl Solution for Day10 {
    fn get_day(&self) -> u8 {
        10
    }

    fn part1(&self, input: String) -> String {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

        let start = find_start(&grid);

        let mut queue: Vec<(usize, (usize, usize), Vec<Vec<bool>>)> =
            vec![(0, (start.0, start.1), visited.clone())];

        let mut finishes = Vec::new();

        while let Some((counter, pos1, mut visited)) = queue.pop() {
            visited[pos1.0][pos1.1] = true;

            if grid[pos1.0][pos1.1] == 'S' && counter != 0 {
                finishes.push(counter);
            }

            let directions = get_valid_neighbors(&grid, &visited, pos1);

            for direction in directions {
                let pos2 = (
                    (pos1.0 as i32 + direction.row()) as usize,
                    (pos1.1 as i32 + direction.col()) as usize,
                );

                if is_connected(grid[pos1.0][pos1.1], grid[pos2.0][pos2.1], direction)
                    || (grid[pos2.0][pos2.1] == 'S' && counter > 1)
                {
                    queue.push((counter + 1, pos2, visited.clone()));
                }
            }
        }

        finishes.sort();

        (finishes[finishes.len() - 1] / 2).to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

        let start = find_start(&grid);

        let mut queue: Vec<(usize, (usize, usize), Vec<Vec<bool>>)> =
            vec![(0, (start.0, start.1), visited.clone())];

        let mut finished_visits = Vec::new();

        while let Some((counter, pos1, mut visited)) = queue.pop() {
            visited[pos1.0][pos1.1] = true;

            if grid[pos1.0][pos1.1] == 'S' && counter != 0 {
                finished_visits.push((counter, visited.clone()));
            }

            let directions = get_valid_neighbors(&grid, &visited, pos1);

            for direction in directions {
                let pos2 = (
                    (pos1.0 as i32 + direction.row()) as usize,
                    (pos1.1 as i32 + direction.col()) as usize,
                );

                if is_connected(grid[pos1.0][pos1.1], grid[pos2.0][pos2.1], direction)
                    || (grid[pos2.0][pos2.1] == 'S' && counter > 1)
                {
                    queue.push((counter + 1, pos2, visited.clone()));
                }
            }
        }

        finished_visits
            .sort_by(|(counter_a, _visited_a), (counter_b, _visited_b)| counter_b.cmp(counter_a));

        for (row, rows) in finished_visits[0].1.iter().enumerate() {
            for (col, value) in rows.iter().enumerate() {
                if !*value {
                    grid[row][col] = '.';
                }
            }
        }

        grid[start.0][start.1] = 'J';

        println!("{:?}", grid[start.0][start.1]);

        let ret = get_enclosed(&mut grid, &finished_visits[0].1).to_string();

        ret.to_string()
    }
}

fn get_enclosed(grid: &mut [Vec<char>], visited: &[Vec<bool>]) -> usize {
    let mut counter = 0;
    let mut enclosed = Vec::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, _value) in row.iter().enumerate() {
            if !visited[row_index][col_index] {
                let mut intersect_counter = 0;
                let mut last = None;

                for index in col_index + 1..row.len() {
                    if grid[row_index][index] == '7' {
                        if let Some(last) = last {
                            if last == 'L' {
                                intersect_counter += 1;
                            }
                        }
                    } else if grid[row_index][index] == 'J' {
                        if let Some(last) = last {
                            if last == 'F' {
                                intersect_counter += 1;
                            }
                        }
                    } else if grid[row_index][index] == 'L' {
                        last = Some('L');
                    } else if grid[row_index][index] == 'F' {
                        last = Some('F');
                    } else if grid[row_index][index] == '|' {
                        intersect_counter += 1;
                    }
                }

                if intersect_counter % 2 == 1 {
                    enclosed.push((row_index, col_index));

                    counter += 1;
                }
            }
        }
    }

    for enclosed in enclosed.iter() {
        grid[enclosed.0][enclosed.1] = 'I';
    }

    counter
}

fn get_valid_neighbors(
    grid: &[Vec<char>],
    visited: &[Vec<bool>],
    pos: (usize, usize),
) -> Vec<Direction> {
    let direction = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    let mut neighbors = Vec::new();

    for dir in direction {
        let row = pos.0 as i32 + dir.row();
        let col = pos.1 as i32 + dir.col();

        if row < 0 || col < 0 {
            continue;
        }

        if let Some(rows) = grid.get(row as usize) {
            if let Some(value) = rows.get(col as usize) {
                if (*value != '.' && !visited[row as usize][col as usize]) || *value == 'S' {
                    neighbors.push(dir);
                }
            }
        }
    }

    neighbors
}

fn is_connected(pipe1: char, pipe2: char, direction: Direction) -> bool {
    match direction {
        Direction::Up => {
            if (pipe2 == '|' || pipe2 == 'F' || pipe2 == '7')
                && (pipe1 == '|' || pipe1 == 'L' || pipe1 == 'J' || pipe1 == 'S')
            {
                return true;
            }

            false
        }
        Direction::Down => {
            if (pipe2 == '|' || pipe2 == 'L' || pipe2 == 'J')
                && (pipe1 == '|' || pipe1 == 'F' || pipe1 == '7' || pipe1 == 'S')
            {
                return true;
            }

            false
        }
        Direction::Left => {
            if (pipe2 == '-' || pipe2 == 'L' || pipe2 == 'F')
                && (pipe1 == '-' || pipe1 == 'J' || pipe1 == '7' || pipe1 == 'S')
            {
                return true;
            }

            false
        }
        Direction::Right => {
            if (pipe2 == '-' || pipe2 == 'J' || pipe2 == '7')
                && (pipe1 == '-' || pipe1 == 'L' || pipe1 == 'F' || pipe1 == 'S')
            {
                return true;
            }

            false
        }
    }
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col == 'S' {
                return (row_index, col_index);
            }
        }
    }

    panic!("No start found");
}
