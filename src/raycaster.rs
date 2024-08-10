use crate::map::Map;
use crate::player::Player;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f32::consts::PI;
use image::RgbaImage;

pub struct RayCaster {
    map: Map,
    pub player: Player,
    wall_texture: RgbaImage,
}

impl RayCaster {
    pub fn new(map: Map, player: Player) -> Result<Self, String> {
        let wall_texture = image::open("wall_texture.png")
            .map_err(|e| e.to_string())?
            .to_rgba8();
        Ok(Self { map, player, wall_texture })
    }

    pub fn rotate_player(&mut self, angle: f32) {
        self.player.rotate(angle);
    }

    pub fn move_player_forward(&mut self, distance: f32) {
        self.player.move_forward(distance);
        self.check_collision();
    }

    pub fn move_player_backward(&mut self, distance: f32) {
        self.player.move_backward(distance);
        self.check_collision();
    }

    fn check_collision(&mut self) {
        let x = self.player.x as usize;
        let y = self.player.y as usize;
        if self.map.get(x, y) != Some(' ') {
            self.player.x = self.player.x.floor() + 0.5;
            self.player.y = self.player.y.floor() + 0.5;
        }
    }

    fn cast_ray(&self, angle: f32) -> (f32, char) {
        let mut x = self.player.x;
        let mut y = self.player.y;
        let dx = angle.cos();
        let dy = angle.sin();

        loop {
            x += dx * 0.1;
            y += dy * 0.1;

            let map_x = x as usize;
            let map_y = y as usize;

            if let Some(cell) = self.map.get(map_x, map_y) {
                if cell != ' ' {
                    let distance = ((x - self.player.x).powi(2) + (y - self.player.y).powi(2)).sqrt();
                    return (distance, cell);
                }
            }
        }
    }

    pub fn render_3d(&self, canvas: &mut Canvas<Window>, width: u32, height: u32) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let num_rays = width;
        let angle_step = self.player.fov / num_rays as f32;

        for (i, ray_angle) in (0..num_rays).map(|i| {
            self.player.angle - self.player.fov / 2.0 + i as f32 * angle_step
        }).enumerate() {
            let (distance, wall_type) = self.cast_ray(ray_angle);

            let wall_height = (height as f32 / distance).min(height as f32);
            let wall_top = (height as f32 - wall_height) / 2.0;

            let texture_x = (self.wall_texture.width() as f32 * (ray_angle / (2.0 * PI))) as u32 % self.wall_texture.width();

            for y in wall_top as u32..(wall_top + wall_height) as u32 {
                let texture_y = ((y - wall_top as u32) as f32 / wall_height * self.wall_texture.height() as f32) as u32;
                let color = self.wall_texture.get_pixel(texture_x, texture_y);
                canvas.set_draw_color(Color::RGBA(color[0], color[1], color[2], color[3]));
                canvas.draw_point(Point::new(i as i32, y as i32)).unwrap();
            }
        }

        canvas.present();
    }

    pub fn render_2d(&self, canvas: &mut Canvas<Window>, width: u32, height: u32) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let cell_size = 20;
        for (y, row) in self.map.data.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell != ' ' {
                    canvas.set_draw_color(Color::WHITE);
                    canvas.fill_rect(Rect::new(
                        (x * cell_size) as i32,
                        (y * cell_size) as i32,
                        cell_size as u32,
                        cell_size as u32,
                    )).unwrap();
                }
            }
        }

        // Draw player
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(Rect::new(
            (self.player.x * cell_size as f32) as i32 - 2,
            (self.player.y * cell_size as f32) as i32 - 2,
            5,
            5,
        )).unwrap();

        canvas.present();
    }
}