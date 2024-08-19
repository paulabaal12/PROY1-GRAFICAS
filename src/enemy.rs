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
    
    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize, player: &Player, z_buffer: &[f64]) {
        let dx = self.x - player.x;
        let dy = self.y - player.y;
        
        let distance = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx) - player.angle;
        
        // Adjust angle to be within -PI to PI
        let angle = (angle + std::f64::consts::PI * 3.0) % (std::f64::consts::PI * 2.0) - std::f64::consts::PI;
    
        // Check if enemy is in front of the player and within field of view
        if distance > 0.5 && angle.abs() < std::f64::consts::FRAC_PI_4 {
            let sprite_size = (height as f64 / distance) as usize;
            let h_offset = ((angle / std::f64::consts::FRAC_PI_4 + 1.0) * width as f64 / 2.0) as i32;
            
            let (tex_width, tex_height) = self.texture.dimensions();
            let texture = self.texture.to_rgba8();
    
            for sx in 0..sprite_size {
                let screen_x = h_offset + sx as i32 - (sprite_size / 2) as i32;
                if screen_x < 0 || screen_x >= width as i32 {
                    continue;
                }
                
                let tex_x = (sx as f64 / sprite_size as f64 * tex_width as f64) as u32;
    
                if z_buffer[screen_x as usize] > distance {
                    for sy in 0..sprite_size {
                        let y = height / 2 + sy - sprite_size / 2;
                        if y >= height {
                            break;
                        }
    
                        let tex_y = (sy as f64 / sprite_size as f64 * tex_height as f64) as u32;
                        let color = texture.get_pixel(tex_x, tex_y);
    
                        if color[3] > 0 {
                            let idx = y * width + screen_x as usize;
                            if idx < buffer.len() {
                                buffer[idx] = u32::from_be_bytes([color[0], color[1], color[2], color[3]]);
                            }
                        }
                    }
                }
            }
        }
    }
}    