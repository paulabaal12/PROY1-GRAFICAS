use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Map {
    data: Vec<Vec<char>>,
}

impl Map {
    pub fn load(filename: &str) -> Self {
        match File::open(filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let data = reader.lines()
                    .map(|line| line.unwrap().chars().collect())
                    .collect();
                Map { data }
            },
            Err(e) => panic!("Failed to open map file '{}': {}", filename, e),
        }
    }

    pub fn is_wall(&self, x: f64, y: f64) -> bool {
        let map_x = x as usize;
        let map_y = y as usize;
        self.get_cell(map_x, map_y) == '+' || self.get_cell(map_x, map_y) == '-' || self.get_cell(map_x, map_y) == '|'
    }

    pub fn get_cell(&self, x: usize, y: usize) -> char {
        self.data.get(y).and_then(|row| row.get(x)).cloned().unwrap_or(' ')
    }

    pub fn find_player_start(&self) -> (f64, f64) {
        for (y, row) in self.data.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 'p' {
                    return (x as f64 + 0.5, y as f64 + 0.5);
                }
            }
        }
        panic!("Player start position not found in map");
    }


    pub fn find_enemy_start(&self) -> (f64, f64) {
        for (y, row) in self.data.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 'E' {
                    return (x as f64 + 0.5, y as f64 + 0.5);
                }
            }
        }
        panic!("Enemy start position not found in map");
    }

    pub fn is_player_at_goal(&self, player: &crate::player::Player) -> bool {
        let map_x = player.x as usize;
        let map_y = player.y as usize;
        self.get_cell(map_x, map_y) == 'g'
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }
}