use image::GenericImageView;
use crate::map::Map;
use crate::player::Player;
use std::f64::consts::PI;

pub struct Renderer {
    width: usize,
    height: usize,
    texture: Vec<u32>,
    img_width: usize,
    img_height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let (texture, img_width, img_height) = Renderer::load_texture("assets/walltexture1.jpg");
        Renderer { width, height, texture, img_width, img_height }
    }

    fn load_texture(filename: &str) -> (Vec<u32>, usize, usize) {
        let img = image::open(filename).expect("Failed to load texture");
        let (img_width, img_height) = img.dimensions();
        let texture = img.to_rgba8().pixels().map(|p| {
            let [r, g, b, a] = p.0;
            ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        }).collect();
        (texture, img_width as usize, img_height as usize)
    }

    fn lerp_color(&self, color1: u32, color2: u32, t: f64) -> u32 {
        let r1 = (color1 >> 16) & 0xFF;
        let g1 = (color1 >> 8) & 0xFF;
        let b1 = color1 & 0xFF;

        let r2 = (color2 >> 16) & 0xFF;
        let g2 = (color2 >> 8) & 0xFF;
        let b2 = color2 & 0xFF;

        let r = self.lerp(r1 as f64, r2 as f64, t) as u32;
        let g = self.lerp(g1 as f64, g2 as f64, t) as u32;
        let b = self.lerp(b1 as f64, b2 as f64, t) as u32;

        (r << 16) | (g << 8) | b
    }

    fn lerp(&self, a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t
    }

    fn sample_texture(&self, u: f64, v: f64) -> u32 {
        let img_width = self.img_width;
        let img_height = self.img_height;

        // Mapeo de coordenadas (u, v) al tamaño de la imagen
        let x = u * img_width as f64;
        let y = v * img_height as f64;

        // Coordenadas de los píxeles vecinos
        let x1 = x.floor() as usize % img_width;
        let y1 = y.floor() as usize % img_height;
        let x2 = (x1 + 1) % img_width;
        let y2 = (y1 + 1) % img_height;

        // Factores de interpolación
        let tx = x - x.floor();
        let ty = y - y.floor();

        // Muestreo bilineal
        let c11 = self.texture[y1 * img_width + x1];
        let c21 = self.texture[y1 * img_width + x2];
        let c12 = self.texture[y2 * img_width + x1];
        let c22 = self.texture[y2 * img_width + x2];

        self.bilinear_interpolation(c11, c21, c12, c22, tx, ty)
    }

    fn bilinear_interpolation(&self, c11: u32, c21: u32, c12: u32, c22: u32, tx: f64, ty: f64) -> u32 {
        let r = self.lerp(
            self.lerp(((c11 >> 16) & 0xFF) as f64, ((c21 >> 16) & 0xFF) as f64, tx),
            self.lerp(((c12 >> 16) & 0xFF) as f64, ((c22 >> 16) & 0xFF) as f64, tx),
            ty,
        ) as u32;
        let g = self.lerp(
            self.lerp(((c11 >> 8) & 0xFF) as f64, ((c21 >> 8) & 0xFF) as f64, tx),
            self.lerp(((c12 >> 8) & 0xFF) as f64, ((c22 >> 8) & 0xFF) as f64, tx),
            ty,
        ) as u32;
        let b = self.lerp(
            self.lerp((c11 & 0xFF) as f64, (c21 & 0xFF) as f64, tx),
            self.lerp((c12 & 0xFF) as f64, (c22 & 0xFF) as f64, tx),
            ty,
        ) as u32;

        (r << 16) | (g << 8) | b
    }

    pub fn render_3d(&self, map: &Map, player: &Player) -> Vec<u32> {
        let mut buffer = vec![0; self.width * self.height];
        let sky_top = 0x87CEEB;
        let sky_bottom = 0xf8eaf7;

        for x in 0..self.width {
            let ray_angle = player.angle - PI / 6.0 + (x as f64 / self.width as f64) * PI / 3.0;
            let (distance, wall_x) = self.cast_ray(map, player, ray_angle);

            let wall_height = (self.height as f64 / distance) as usize;
            let wall_top = (self.height / 2).saturating_sub(wall_height / 2);
            let wall_bottom = (self.height / 2 + wall_height / 2).min(self.height);

            let texture_u = wall_x;
            let texture_step = 1.0 / wall_height as f64;
            let mut texture_v = 0.0;

            for y in 0..self.height {
                let pixel_index = y * self.width + x;
                if y < wall_top {
                    let t = y as f64 / wall_top as f64;
                    buffer[pixel_index] = self.color_lerp(sky_top, sky_bottom, t);
                } else if y >= wall_top && y < wall_bottom {
                    let color = self.sample_texture(texture_u, texture_v);
                    let shade_factor = 1.0 / (1.0 + distance * distance * 0.1);
                    buffer[pixel_index] = self.apply_shade(color, shade_factor);
                    texture_v += texture_step;
                } else {
                    let t = (y - wall_bottom) as f64 / (self.height - wall_bottom) as f64;
                    buffer[pixel_index] = self.color_lerp(0x0d798f, 0x4f0955, t);
                }
            }
        }
        buffer
    }

    fn apply_shade(&self, color: u32, factor: f64) -> u32 {
        let r = ((color >> 16) & 0xFF) as f64 * factor;
        let g = ((color >> 8) & 0xFF) as f64 * factor;
        let b = (color & 0xFF) as f64 * factor;
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    fn cast_ray(&self, map: &Map, player: &Player, angle: f64) -> (f64, f64) {
        let mut x = player.x;
        let mut y = player.y;
        let step_size = 0.01;
        let dx = angle.cos() * step_size;
        let dy = angle.sin() * step_size;

        loop {
            x += dx;
            y += dy;

            if map.is_wall(x, y) {
                let distance = ((x - player.x).powi(2) + (y - player.y).powi(2)).sqrt();
                let wall_x = x - x.floor();
                return (distance, wall_x);
            }
        }
    }

    pub fn render_minimap(&self, map: &Map, player: &crate::player::Player, buffer: &mut Vec<u32>) {
        let minimap_size = 140;
        let scale = minimap_size as f64 / map.width() as f64;

        for y in 0..minimap_size {
            for x in 0..minimap_size {
                let map_x = (x as f64 / scale) as usize;
                let map_y = (y as f64 / scale) as usize;

                if map_x < map.width() && map_y < map.height() {
                    let color = match map.get_cell(map_x, map_y) {
                        ' ' => 0xF6F5F2,
                        '+' | '-' | '|' => 0x4B4C60,
                        'P' => 0x46A1C9,
                        _ => 0x000000,
                    };
                    let pixel_index = (y * self.width + x) as usize;
                    buffer[pixel_index] = color;
                }
            }
        }

        let player_x = (player.x * scale) as usize;
        let player_y = (player.y * scale) as usize;

        for dy in 0..3 {
            for dx in 0..3 {
                let pixel_index = ((player_y + dy) * self.width + (player_x + dx)) as usize;
                if pixel_index < buffer.len() {
                    buffer[pixel_index] = 0xFF0000;
                }
            }
        }
    }

    fn color_lerp(&self, start: u32, end: u32, t: f64) -> u32 {
        let r1 = (start >> 16) & 0xFF;
        let g1 = (start >> 8) & 0xFF;
        let b1 = start & 0xFF;

        let r2 = (end >> 16) & 0xFF;
        let g2 = (end >> 8) & 0xFF;
        let b2 = end & 0xFF;

        let r = (r1 as f64 + (r2 as f64 - r1 as f64) * t) as u32;
        let g = (g1 as f64 + (g2 as f64 - g1 as f64) * t) as u32;
        let b = (b1 as f64 + (b2 as f64 - b1 as f64) * t) as u32;

        (r << 16) | (g << 8) | b
    }
}
