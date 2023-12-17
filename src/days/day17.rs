use crate::api::{Matrix2D, Solution};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct SearchState {
    position: (i32, i32),
    dir: (i32, i32),
    straight: usize,
}

pub struct Day17;

impl Solution for Day17 {
    fn get_day(&self) -> u8 {
        17
    }

    fn part1(&self, input: String) -> String {
        let map = Matrix2D::from(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c as usize - '0' as usize)
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
        );

        let end = (map.rows() as i32 - 1, map.cols() as i32 - 1);
        let start = SearchState::default();

        let result = solve(&map, start, end, 0, 3).unwrap();

        result.to_string()
    }

    fn part2(&self, input: String) -> String {
        let map = Matrix2D::from(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c as usize - '0' as usize)
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
        );

        let end = (map.rows() as i32 - 1, map.cols() as i32 - 1);
        let start = SearchState::default();

        let result = solve(&map, start, end, 4, 10).unwrap();

        result.to_string()
    }
}

fn solve(
    map: &Matrix2D<usize>,
    start: SearchState,
    end: (i32, i32),
    min: usize,
    max: usize,
) -> Option<usize> {
    let mut heuristics = Matrix2D::new(0, (map.rows(), map.cols()));
    heuristics
        .iter_enumerate_mut()
        .for_each(|((row, col), value)| {
            *value = manhattan_distance((row as i32, col as i32), end);
        });

    let mut queue = Vec::new();
    queue.push((0, heuristics.get(&start.position).unwrap(), start));

    let mut visited: Vec<SearchState> = Vec::new();

    while let Some((heat_loss, _, state)) = queue.pop() {
        if visited.contains(&state) {
            continue;
        }

        visited.push(state.clone());

        if state.position == end && state.straight >= min {
            return Some(heat_loss);
        }

        if state.straight < max && state.dir != (0, 0) {
            let mut new_state = state.clone();
            new_state.position = (
                state.position.0 + state.dir.0,
                state.position.1 + state.dir.1,
            );

            if let Some(loss) = map.get(&new_state.position) {
                new_state.straight += 1;
                queue.push((
                    heat_loss + loss,
                    heuristics.get(&state.position).unwrap(),
                    new_state,
                ));
            }
        }

        if state.straight >= min || state.dir == (0, 0) {
            for direction in [(0, 1), (1, 0), (0, -1), (-1, 0)]
                .iter()
                .filter(|(row, col)| state.dir != (-row, -col) && (state.dir != (*row, *col)))
            {
                let mut new_state = state.clone();
                new_state.position = (
                    state.position.0 + direction.0,
                    state.position.1 + direction.1,
                );

                if let Some(loss) = map.get(&new_state.position) {
                    new_state.dir = *direction;
                    new_state.straight = 1;
                    queue.push((
                        heat_loss + loss,
                        heuristics.get(&state.position).unwrap(),
                        new_state,
                    ));
                }
            }
        }

        //println!("{:?}", queue);
        queue.sort_by(|a, b| (b.1 + b.0).cmp(&(a.0 + a.1)));
    }

    None
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> usize {
    ((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as usize
}
