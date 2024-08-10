use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub struct Map {
    pub data: Vec<Vec<char>>,
}

impl Map {
    pub fn new(filename: &str) -> Self {
        Self {
            data: load_maze(filename),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y).and_then(|row| row.get(x).cloned())
    }
}