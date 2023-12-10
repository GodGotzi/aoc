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

pub type IVec2 = (i32, i32);

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn get(&self, (row, col): &(i32, i32)) -> Option<&T> {
        if *row < 0 || *col < 0 {
            return None;
        }

        self.data
            .get(*row as usize)
            .and_then(|row| row.get(*col as usize))
    }

    pub fn get_mut(&mut self, (row, col): &(i32, i32)) -> Option<&mut T> {
        if *row < 0 || *col < 0 {
            return None;
        }

        self.data
            .get_mut(*row as usize)
            .and_then(|row| row.get_mut(*col as usize))
    }

    pub fn set(&mut self, (row, col): &(i32, i32), value: T) {
        if *row < 0 || *col < 0 {
            return;
        }

        if let Some(row) = self.data.get_mut(*row as usize) {
            if let Some(old) = row.get_mut(*col as usize) {
                *old = value.clone();
            }
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().flatten()
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.data.iter().enumerate().flat_map(|(row, values)| {
            values
                .iter()
                .enumerate()
                .map(move |(col, value)| ((row, col), value))
        })
    }
}
