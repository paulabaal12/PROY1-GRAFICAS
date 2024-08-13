use crate::map::Map;
use std::f64::consts::PI;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

impl Player {
    pub fn new(map: &Map) -> Self {
        let (x, y) = map.find_player_start();
        Player { x, y, angle: 0.0 }
    }

    pub fn move_forward(&mut self, map: &Map) {
        let new_x = self.x + self.angle.cos() * 0.05;
        let new_y = self.y + self.angle.sin() * 0.05;
        if !map.is_wall(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn move_backward(&mut self, map: &Map) {
        let new_x = self.x - self.angle.cos() * 0.05;
        let new_y = self.y - self.angle.sin() * 0.05;
        if !map.is_wall(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn rotate_left(&mut self) {
        self.angle -= 0.05;
        if self.angle < 0.0 {
            self.angle += 2.0 * PI;
        }
    }

    pub fn rotate_right(&mut self) {
        self.angle += 0.05;
        if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }

    pub fn rotate(&mut self, amount: f64) {
        self.angle += amount;
        if self.angle < 0.0 {
            self.angle += 2.0 * PI;
        } else if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }
}