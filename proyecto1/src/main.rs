use minifb::{Key, Window, WindowOptions};
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};
use image::GenericImageView;

const WIDTH: usize = 840;
const HEIGHT: usize = 580;

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

fn color_lerp(color1: u32, color2: u32, t: f64) -> u32 {
    let r1 = (color1 >> 16) & 0xFF;
    let g1 = (color1 >> 8) & 0xFF;
    let b1 = color1 & 0xFF;

    let r2 = (color2 >> 16) & 0xFF;
    let g2 = (color2 >> 8) & 0xFF;
    let b2 = color2 & 0xFF;

    let r = lerp(r1 as f64, r2 as f64, t) as u32;
    let g = lerp(g1 as f64, g2 as f64, t) as u32;
    let b = lerp(b1 as f64, b2 as f64, t) as u32;

    (r << 16) | (g << 8) | b
}

struct Player {
    x: f64,
    y: f64,
    angle: f64,
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Ray Caster",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let map = load_map("assets/maze.txt");
   let texture = load_texture("assets/walltexture.jpg");
    let (mut player, map_width, map_height) = find_player(&map);

    let mut view_mode = 0; // 0 for 3D, 1 for 2D

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::A) {
            player.angle -= 0.05;
        }
        if window.is_key_down(Key::D) {
            player.angle += 0.05;
        }
        if window.is_key_down(Key::W) {
            let new_x = player.x + player.angle.cos() * 0.05;
            let new_y = player.y + player.angle.sin() * 0.05;
            if !is_wall(&map, new_x, new_y) {
                player.x = new_x;
                player.y = new_y;
            }
        }
        if window.is_key_down(Key::S) {
            let new_x = player.x - player.angle.cos() * 0.05;
            let new_y = player.y - player.angle.sin() * 0.05;
            if !is_wall(&map, new_x, new_y) {
                player.x = new_x;
                player.y = new_y;
            }
        }
        if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
            view_mode = 1 - view_mode;
        }

        if view_mode == 0 {
            render_3d(&mut buffer, &map, &player, &texture, map_width, map_height);
        } else {
            render_2d(&mut buffer, &map, &player, map_width, map_height);
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn load_map(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Failed to open map file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap().chars().collect()).collect()
}

fn load_texture(filename: &str) -> Vec<u32> {
    let img = image::open(filename).expect("Failed to load texture");
    let (width, height) = img.dimensions();
    img.to_rgba8().pixels().map(|p| {
        let [r, g, b, a] = p.0;
        ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }).collect()
}

fn find_player(map: &Vec<Vec<char>>) -> (Player, usize, usize) {
    let height = map.len();
    let width = map[0].len();
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'p' {
                return (
                    Player {
                        x: x as f64 + 0.5,
                        y: y as f64 + 0.5,
                        angle: 0.0,
                    },
                    width,
                    height,
                );
            }
        }
    }
    panic!("Player not found in map");
}

fn is_wall(map: &Vec<Vec<char>>, x: f64, y: f64) -> bool {
    let map_x = x as usize;
    let map_y = y as usize;
    map[map_y][map_x] == '+' || map[map_y][map_x] == '-' || map[map_y][map_x] == '|'
}

fn render_2d(buffer: &mut Vec<u32>, map: &Vec<Vec<char>>, player: &Player, map_width: usize, map_height: usize) {
    for (i, pixel) in buffer.iter_mut().enumerate() {
        let x = i % WIDTH;
        let y = i / WIDTH;
        let map_x = x * map_width / WIDTH;
        let map_y = y * map_height / HEIGHT;

        *pixel = match map[map_y][map_x] {
            ' ' => 0xF6F5F2,
            '+' | '-' | '|' => 0x0d798f,
            'g' => 0xFF0000,
            _ => 0x000000,
        };
    }

    let player_screen_x = (player.x * WIDTH as f64 / map_width as f64) as usize;
    let player_screen_y = (player.y * HEIGHT as f64 / map_height as f64) as usize;
    for dy in -2..=2 {
        for dx in -2..=2 {
            let px = player_screen_x as i32 + dx;
            let py = player_screen_y as i32 + dy;
            if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                buffer[py as usize * WIDTH + px as usize] = 0x500CFF;
            }
        }
    }
}

fn render_3d(buffer: &mut Vec<u32>, map: &Vec<Vec<char>>, player: &Player, texture: &Vec<u32>, map_width: usize, map_height: usize) {
    let sky_top = 0x87CEEB;     // Azul claro
    let sky_bottom = 0xf8eaf7;  // Muy claro

    for x in 0..WIDTH {
        let ray_angle = player.angle - PI / 6.0 + (x as f64 / WIDTH as f64) * PI / 3.0;
        let (distance, wall_x) = cast_ray(map, player, ray_angle);

        let wall_height = (HEIGHT as f64 / distance) as usize;
        let wall_top = (HEIGHT / 2).saturating_sub(wall_height / 2);
        let wall_bottom = (HEIGHT / 2 + wall_height / 2).min(HEIGHT);

        let texture_x = (wall_x * 64.0) as usize & 63;

        for y in 0..HEIGHT {
            let pixel_index = y * WIDTH + x;
            if y < wall_top {
                // Cielo con degradado
                let t = y as f64 / wall_top as f64;
                buffer[pixel_index] = color_lerp(sky_top, sky_bottom, t);
            } else if y >= wall_top && y < wall_bottom {
                let texture_y = (y - wall_top) * 64 / wall_height;
                buffer[pixel_index] = texture[texture_y * 64 + texture_x];
            } else {
                // Suelo con nuevos tonos
                let t = (y - wall_bottom) as f64 / (HEIGHT - wall_bottom) as f64;
                buffer[pixel_index] = color_lerp(0x0d798f, 0x4f0955, t);  // De un azul oscuro a un azul más claro
            }
        }
    }
}


fn cast_ray(map: &Vec<Vec<char>>, player: &Player, angle: f64) -> (f64, f64) {
    let mut x = player.x;
    let mut y = player.y;
    let step_size = 0.01;
    let dx = angle.cos() * step_size;
    let dy = angle.sin() * step_size;

    loop {
        x += dx;
        y += dy;

        let map_x = x as usize;
        let map_y = y as usize;

        if map_x >= map[0].len() || map_y >= map.len() {
            return (f64::MAX, 0.0);
        }

        if is_wall(map, x, y) {
            let distance = ((x - player.x).powi(2) + (y - player.y).powi(2)).sqrt();
            let wall_x = x - x.floor(); // Used for texture mapping
            return (distance, wall_x);
        }
    }
}