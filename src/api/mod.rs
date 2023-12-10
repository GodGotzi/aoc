pub trait Solution {
    fn load_input(&self) -> String {
        let path = format!("input/day{:02}", self.get_day());
        println!("Loading input from: {}", path);
        std::fs::read_to_string(path).expect("Something went wrong reading the file")
    }

    fn get_day(&self) -> u8;
    fn part1(&self, input: String) -> String;
    fn part2(&self, input: String) -> String;
}

pub struct Matrix2D<T> {
    data: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for Matrix2D<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }
}

impl<T: Clone> Matrix2D<T> {
    pub fn new(default: T, size: (usize, usize)) -> Self {
        Self {
            data: vec![vec![default; size.1]; size.0],
        }
    }

    pub fn get(&self, row: i32, col: i32) -> Option<&T> {
        if row < 0 || col < 0 {
            return None;
        }

        self.data
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flatten()
    }
}
