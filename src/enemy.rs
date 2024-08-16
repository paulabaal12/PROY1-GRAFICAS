use crate::map::Map;
use crate::player::Player;
use minifb::Window;
use image::{DynamicImage, GenericImageView}; 

pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub texture: DynamicImage,
    pub speed: f64,
}

impl Enemy {
    pub fn new(map: &Map) -> Self {
        let (x, y) = map.find_enemy_start();
        let texture = image::open("assets/enemy.png").unwrap();
        let speed = 0.1;
    
        Enemy { x, y, texture, speed }
    }
    

    pub fn update(&mut self, map: &Map, player: &Player, dt: f64) {
        let dx = player.x - self.x;
        let dy = player.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            let move_x = dx / distance * self.speed * dt;
            let move_y = dy / distance * self.speed * dt;

            let new_x = self.x + move_x;
            let new_y = self.y + move_y;

            if !map.is_wall(new_x, new_y) {
                self.x = new_x;
                self.y = new_y;
            }
        }
    }
    
    pub fn has_caught_player(&self, player: &Player) -> bool {
        let distance = ((self.x - player.x).powi(2) + (self.y - player.y).powi(2)).sqrt();
        distance < 0.8
    }

    pub fn render(&self, window: &mut Window, buffer: &mut Vec<u32>, width: usize, height: usize) {
        let (enemy_width, enemy_height) = self.texture.dimensions();
        let enemy_x = (self.x * width as f64) as isize;
        let enemy_y = (self.y * height as f64) as isize;
    
        for y in 0..enemy_height as isize {
            for x in 0..enemy_width as isize {
                let buffer_x = enemy_x + x;
                let buffer_y = enemy_y + y;
    
                if buffer_x >= 0 && buffer_x < width as isize && buffer_y >= 0 && buffer_y < height as isize {
                    let pixel = self.texture.get_pixel(x as u32, y as u32);
                    let rgba = pixel.0;
                    let color = (rgba[0] as u32) << 16 | (rgba[1] as u32) << 8 | rgba[2] as u32;
                    buffer[buffer_y as usize * width + buffer_x as usize] = color;
                }
            }
        }
    }
    
}

