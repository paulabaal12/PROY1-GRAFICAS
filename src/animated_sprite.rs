use std::time::{Duration, Instant};
use image::{ RgbaImage};
use std::fs;

pub struct AnimatedSprite {
    frames: Vec<RgbaImage>,
    frame_duration: Duration,
    current_frame: usize,
    last_update: Instant,
}

impl AnimatedSprite {
    pub fn new(path: &str, frame_count: usize, frame_duration: Duration) -> Result<Self, String> {
        let mut frames = Vec::with_capacity(frame_count);
        for i in 0..frame_count {
            let file_path = format!("{}_{}.png", path, i); // Ajusta esto si es necesario
            println!("Attempting to open: {}", file_path); // Añade esto para depurar
            if !fs::metadata(&file_path).is_ok() {
                return Err(format!("File not found: {}", file_path));
            }
            let img = image::open(file_path).map_err(|e| e.to_string())?;
            frames.push(img.to_rgba8()); 
        }
    
        Ok(AnimatedSprite {
            frames,
            frame_duration,
            current_frame: 0,
            last_update: Instant::now(),
        })
    }

    pub fn update(&mut self) {
        if self.last_update.elapsed() >= self.frame_duration {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_update = Instant::now();
        }
    }
    
    pub fn render(&self, buffer: &mut [u32], width: usize, height: usize) {
        let frame = &self.frames[self.current_frame];
        let (frame_width, frame_height) = frame.dimensions();

        let x_offset = (width as u32 - frame_width) / 2;
        let y_offset = height as u32 - frame_height;

        for y in 0..frame_height {
            for x in 0..frame_width {
                let px = frame.get_pixel(x, y);
                let alpha = px[3];
                let idx = ((y + y_offset) as usize * width + (x + x_offset) as usize) % (width * height);

                if alpha > 0 {
                    buffer[idx] = px[0] as u32 | ((px[1] as u32) << 8) | ((px[2] as u32) << 16) | ((px[3] as u32) << 24);
                }
            }
        }
    }
}
