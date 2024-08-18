use image::{DynamicImage, GenericImageView, RgbaImage, Rgba};
use crate::map::Map;
use crate::player::Player;
use glam::Vec2;

pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub texture: DynamicImage,
    pub speed: f64,
}

impl Enemy {
    pub fn new(map: &Map) -> Self {
        let (x, y) = map.find_enemy_start();
        let texture = match image::open("assets/enemy.png") {
            Ok(img) => img,
            Err(e) => {
                println!("Error loading enemy texture: {:?}", e);
                DynamicImage::new_rgba8(0, 0)
            }
        };
        let speed = 0.0;
        
        Enemy { x, y, texture, speed }
    }
    

    pub fn update(&mut self, _map: &Map, _player: &Player, _dt: f64) {
        
    }

    pub fn has_caught_player(&self, player: &Player) -> bool {
        let distance = ((self.x - player.x).powi(2) + (self.y - player.y).powi(2)).sqrt();
        distance < 0.3 
    }
    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize, _player: &Player, _z_buffer: &[f64]) {
        let (tex_width, tex_height) = self.texture.dimensions();
        let texture = self.texture.to_rgba8();
    
        let center_x = (self.x * width as f64) as i32;
        let center_y = (self.y * height as f64) as i32;
    
        for y in 0..tex_height {
            for x in 0..tex_width {
                let pixel = texture.get_pixel(x, y);
                let tx = center_x + x as i32 - tex_width as i32 / 2;
                let ty = center_y + y as i32 - tex_height as i32 / 2;
    
                if tx >= 0 && tx < width as i32 && ty >= 0 && ty < height as i32 {
                    let index = (ty as usize * width + tx as usize) as usize;
                    if pixel[3] > 0 { 
                        let color = u32::from_be_bytes([pixel[0], pixel[1], pixel[2], pixel[3]]);
                        buffer[index] = color;
                    }
                }
            }
        }
    }
}    