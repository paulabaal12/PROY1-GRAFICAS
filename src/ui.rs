use minifb::Window;
use image::RgbaImage;

pub struct UI {
    welcome_image: RgbaImage,
    victory_image: RgbaImage,
    game_over_image: RgbaImage,
}

impl UI {
    pub fn new() -> Self {
        let welcome_image = image::open("assets/welcome.png").unwrap().to_rgba8();
        let victory_image = image::open("assets/victory.png").unwrap().to_rgba8();
        let game_over_image = image::open("assets/gameover.png").unwrap().to_rgba8();
        UI { welcome_image, victory_image, game_over_image }
    }

    pub fn show_welcome_screen(&self, window: &mut Window) {
        self.show_image(window, &self.welcome_image);
    }

    pub fn show_victory_screen(&self, window: &mut Window) {
        self.show_image(window, &self.victory_image);
    }

    pub fn show_game_over_screen(&self, window: &mut Window) {
        self.show_image(window, &self.game_over_image);
    }

    fn show_image(&self, window: &mut Window, image: &RgbaImage) {
        let (width, height) = window.get_size();
        let mut buffer = vec![0; width * height];
        
        let resized_image = image::imageops::resize(image, width as u32, height as u32, image::imageops::FilterType::Nearest);

        self.draw_image(&mut buffer, width, &resized_image);
        
        window.update_with_buffer(&buffer, width, height).unwrap();
    }

    pub fn render_fps(&self, fps: u32, buffer: &mut [u32], width: usize, height: usize) {
        let fps_text = format!("FPS: {}", fps);
        let text_color = 0xFFFFFF; 
        let background_color = 0x000000; 
        let scale = 2; // Aumenta este valor para hacer el texto más grande
    
        let x = width - (fps_text.len() * 8 * scale) - 10; 
        let y = 10; 
    
        for (i, c) in fps_text.chars().enumerate() {
            self.draw_char_scaled(buffer, width, c, x + i * 8 * scale, y, text_color, background_color, scale);
        }
    }
    
    fn draw_char_scaled(&self, buffer: &mut [u32], width: usize, c: char, x: usize, y: usize, color: u32, bg_color: u32, scale: usize) {
        let font_char = get_font_data(c);
        for (dy, &row) in font_char.iter().enumerate() {
            for dx in 0..8 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        let pixel_x = x + dx * scale + sx;
                        let pixel_y = y + dy * scale + sy;
                        let index = pixel_y * width + pixel_x;
                        if index < buffer.len() {
                            buffer[index] = if (row & (1 << (7 - dx))) != 0 { color } else { bg_color };
                        }
                    }
                }
            }
        }
    }
    
    fn draw_char(&self, buffer: &mut [u32], width: usize, c: char, x: usize, y: usize, color: u32, bg_color: u32) {
        let font_char = get_font_data(c);
        for (dy, &row) in font_char.iter().enumerate() {
            for dx in 0..8 {
                let pixel_x = x + dx;
                let pixel_y = y + dy;
                let index = pixel_y * width + pixel_x;
                if index < buffer.len() {
                    buffer[index] = if (row & (1 << (7 - dx))) != 0 { color } else { bg_color };
                }
            }
        }
    }

    fn draw_image(&self, buffer: &mut Vec<u32>, width: usize, image: &RgbaImage) {
        for (x, y, pixel) in image.enumerate_pixels() {
            if x < width as u32 && y < (buffer.len() / width) as u32 {
                let index = y as usize * width + x as usize;
                buffer[index] = ((pixel[3] as u32) << 24) | ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
            }
        }
    }

    fn draw_text(&self, buffer: &mut Vec<u32>, width: usize, text: &str, x: usize, y: usize, color: u32) {
        for (i, c) in text.chars().enumerate() {
            self.draw_char(buffer, width, c, x + i * 8, y, color, 0); 
        }
    }
}

fn get_font_data(c: char) -> [u8; 8] {
    match c {
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11, 0x00],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E, 0x00],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E, 0x00],
        'D' => [0x1E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1E, 0x00],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F, 0x00],
        'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10, 0x00],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0E, 0x00],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11, 0x00],
        'I' => [0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E, 0x00],
        'J' => [0x01, 0x01, 0x01, 0x01, 0x11, 0x11, 0x0E, 0x00],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11, 0x00],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F, 0x00],
        'M' => [0x11, 0x1B, 0x15, 0x11, 0x11, 0x11, 0x11, 0x00],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11, 0x00],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E, 0x00],
        'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10, 0x00],
        'Q' => [0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D, 0x00],
        'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11, 0x00],
        'S' => [0x0E, 0x11, 0x10, 0x0E, 0x01, 0x11, 0x0E, 0x00],
        'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E, 0x00],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04, 0x00],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1B, 0x11, 0x00],
        'X' => [0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11, 0x00],
        'Y' => [0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04, 0x00],
        'Z' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F, 0x00],
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E, 0x00],
        '1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E, 0x00],
        '2' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F, 0x00],
        '3' => [0x0E, 0x11, 0x01, 0x06, 0x01, 0x11, 0x0E, 0x00],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02, 0x00],
        '5' => [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E, 0x00],
        '6' => [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E, 0x00],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08, 0x00],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E, 0x00],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C, 0x00],
        ':' => [0x00, 0x0C, 0x0C, 0x00, 0x0C, 0x0C, 0x00, 0x00],
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        _ => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}