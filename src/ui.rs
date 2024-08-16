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

    pub fn render_fps(&self, fps: u32, buffer: &mut Vec<u32>, width: usize) {
        let fps_text = format!("FPS: {}", fps);
        self.draw_text(buffer, width, &fps_text, 10, 20, 0xFFFFFF);
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
            self.draw_char(buffer, width, c, x + i * 10, y, color);
        }
    }

    fn draw_char(&self, buffer: &mut Vec<u32>, width: usize, c: char, x: usize, y: usize, color: u32) {
        let font = get_font_data(c);
        for (dy, row) in font.iter().enumerate() {
            for dx in 0..8 {
                if (row & (1 << (7 - dx))) != 0 {
                    let pixel_x = x + dx;
                    let pixel_y = y + dy;
                    if pixel_x < width && pixel_y < buffer.len() / width {
                        buffer[pixel_y * width + pixel_x] = color;
                    }
                }
            }
        }
    }
}

// Función auxiliar para obtener datos de fuente (simplificada)
fn get_font_data(c: char) -> [u8; 8] {
    match c {
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11, 0x00],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E, 0x00],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E, 0x00],
        // ... Añade más caracteres según sea necesario
        _ => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}
