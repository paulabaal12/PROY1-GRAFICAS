use std::f32::consts::PI;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub fov: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            angle: PI / 3.0,
            fov: PI / 3.0,
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        self.angle += angle;
        if self.angle < 0.0 {
            self.angle += 2.0 * PI;
        }
        if self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.x += self.angle.cos() * distance;
        self.y += self.angle.sin() * distance;
    }

    pub fn move_backward(&mut self, distance: f32) {
        self.x -= self.angle.cos() * distance;
        self.y -= self.angle.sin() * distance;
    }
}