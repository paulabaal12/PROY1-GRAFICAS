use crate::map::Map;
use std::f64::consts::PI;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    rotation_speed: f64,
}

impl Player {
    pub fn new(map: &Map) -> Self {
        let (x, y) = map.find_player_start();
        Player { x, y, angle: 0.0, rotation_speed: 0.0 }
    }

    pub fn update(&mut self, dt: f64) {
        self.angle += self.rotation_speed * dt;
        self.rotation_speed *= 0.9; 

        if self.angle < 0.0 {
            self.angle += 2.0 * PI;
        } else if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }

    pub fn move_forward(&mut self, map: &Map, speed: f64) {
        let new_x = self.x + self.angle.cos() * speed;
        let new_y = self.y + self.angle.sin() * speed;
        if !map.is_wall(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn move_backward(&mut self, map: &Map, speed: f64) {
        let new_x = self.x - self.angle.cos() * speed;
        let new_y = self.y - self.angle.sin() * speed;
        if !map.is_wall(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn strafe_left(&mut self, map: &Map, speed: f64) {
        let strafe_angle = self.angle - std::f64::consts::PI / 2.0;
        let new_x = self.x + strafe_angle.cos() * speed;
        let new_y = self.y + strafe_angle.sin() * speed;
        if !map.is_wall(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn strafe_right(&mut self, map: &Map, speed: f64) {
        let strafe_angle = self.angle + std::f64::consts::PI / 2.0;
        let new_x = self.x + strafe_angle.cos() * speed;
        let new_y = self.y + strafe_angle.sin() * speed;
        if !map.is_wall(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        self.angle += angle;
        if self.angle < 0.0 {
            self.angle += 2.0 * std::f64::consts::PI;
        } else if self.angle >= 2.0 * std::f64::consts::PI {
            self.angle -= 2.0 * std::f64::consts::PI;
        }
}

} 