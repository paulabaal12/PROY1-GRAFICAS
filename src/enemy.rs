use crate::map::Map;
use crate::player::Player;
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
        let dx = self.x - player.x;
        let dy = self.y - player.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < 0.5 // Ajusta este valor según sea necesario
    }
    
    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize, player: &Player) {
        let dx = self.x - player.x;
        let dy = self.y - player.y;
        let distance = (dx * dx + dy * dy).sqrt();
    
        if distance < 5.0 {  // Solo renderiza si está cerca del jugador
            let angle = dy.atan2(dx) - player.angle;
            let size = (height as f64 / distance).min(height as f64) as usize;
            let x = ((angle.sin() + 1.0) * width as f64 / 2.0) as usize;
    
            let (enemy_width, enemy_height) = self.texture.dimensions();
            for y in 0..size {
                for dx in 0..size {
                    let texture_x = (dx as f64 / size as f64 * enemy_width as f64) as u32;
                    let texture_y = (y as f64 / size as f64 * enemy_height as f64) as u32;
                    let color = self.texture.get_pixel(texture_x, texture_y);
                    let buffer_x = x + dx;
                    let buffer_y = height / 2 + y - size / 2;
                    if buffer_x < width && buffer_y < height && color[3] > 0 {  
                        let index = buffer_y * width + buffer_x;
                        buffer[index] = ((color[3] as u32) << 24) | ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | (color[2] as u32);
                    }
                }
            }
        }
    }
}