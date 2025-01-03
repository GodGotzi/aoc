pub trait Solution {
    fn load_input(&self) -> String {
        let path = format!("input/y{}/day{:02}", self.get_year(), self.get_day());
        println!("Loading input from: {}", path);
        std::fs::read_to_string(path).expect("Something went wrong reading the file")
    }

    fn get_day(&self) -> u8;
    fn get_year(&self) -> u16 {
        2023
    }

    fn part1(&self, input: String) -> String;
    fn part2(&self, input: String) -> String;
}

pub trait IndexString {
    fn index(&self, index: usize) -> char;
}

impl IndexString for String {
    fn index(&self, index: usize) -> char {
        self.chars().nth(index).unwrap()
    }
}

impl IndexString for &str {
    fn index(&self, index: usize) -> char {
        self.chars().nth(index).unwrap()
    }
}

pub type IVec2 = (i32, i32);
pub type Vec2 = (u32, u32);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

    pub fn get_row(&self, row: usize) -> Option<&Vec<T>> {
        self.data.get(row)
    }

    pub fn get_col(&self, col: usize) -> Option<Vec<&T>> {
        self.data.iter().map(|row| row.get(col)).collect()
    }

    pub fn get_row_into_string(&self, row: usize) -> Option<String>
    where
        T: std::fmt::Display,
    {
        self.data
            .get(row)
            .map(|row| row.iter().map(|value| value.to_string()).collect())
    }

    pub fn get_col_into_string(&self, col: usize) -> Option<String>
    where
        T: std::fmt::Display + Clone,
    {
        Some(
            self.data
                .iter()
                .map(|row| row[col].clone())
                .map(|value| value.to_string())
                .collect(),
        )
    }

    pub fn insert_row(&mut self, row: usize, value: Vec<T>) {
        self.data.insert(row, value);
    }

    pub fn insert_col(&mut self, col: usize, value: Vec<T>) {
        for (row, value) in self.data.iter_mut().zip(value) {
            row.insert(col, value);
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

    pub fn iter_enumerate_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.data.iter_mut().enumerate().flat_map(|(row, values)| {
            values
                .iter_mut()
                .enumerate()
                .map(move |(col, value)| ((row, col), value))
        })
    }
}
