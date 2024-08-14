mod player;
mod map;
mod renderer;
mod audio;
mod ui;

use minifb::{Key, Window, WindowOptions};
use std::time::{Instant, Duration};

use player::Player;
use map::Map;
use renderer::Renderer;
use audio::AudioManager;
use ui::UI;

const WIDTH: usize = 840;
const HEIGHT: usize = 580;

fn main() {
    let mut window = Window::new(
        "Ray Caster",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| { panic!("{}", e); });

    window.set_cursor_visibility(false);

    let map = Map::load("assets/maze.txt");
    let mut player = Player::new(&map);
    let renderer = Renderer::new(WIDTH, HEIGHT);
    let audio = AudioManager::new();
    let ui = UI::new();

    audio.play_background_music("assets/nobodynocrimets.mp3");

    let mut last_frame_time = Instant::now();
    let mut fps_counter = 0;
    let mut fps = 0;
    let mut last_step_time = Instant::now();
    let mut view_mode = 0;

    // Pantalla de bienvenida
    ui.show_welcome_screen(&mut window);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();

        // Manejo de entrada
        let mut moved = false;
        if window.is_key_down(Key::W) {
            let new_x = player.x + player.angle.cos() * 0.05;
            let new_y = player.y + player.angle.sin() * 0.05;
            if !map.is_wall(new_x, new_y) {
                player.x = new_x;
                player.y = new_y;
            }
            moved = true;
        }
        if window.is_key_down(Key::S) {
            let new_x = player.x - player.angle.cos() * 0.05;
            let new_y = player.y - player.angle.sin() * 0.05;
            if !map.is_wall(new_x, new_y) {
                player.x = new_x;
                player.y = new_y;
            }
        }
        if window.is_key_down(Key::A) {
            player.rotate_left();
        }
        if window.is_key_down(Key::D) {
            player.rotate_right();
        }

        // Sonido de pasos
        if moved && frame_start.duration_since(last_step_time) >= Duration::from_millis(500) {
            audio.play_footstep();
            last_step_time = frame_start;
        }

        // Renderizado
        let mut buffer = renderer.render_3d(&map, &player);
        renderer.render_minimap(&map, &player, &mut buffer);
        ui.render_fps(fps, &mut buffer, WIDTH);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Actualizar FPS
        fps_counter += 1;
        if frame_start.duration_since(last_frame_time) >= Duration::from_secs(1) {
            fps = fps_counter;
            fps_counter = 0;
            last_frame_time = frame_start;
        }

        // Verificar condición de victoria
        if map.is_player_at_goal(&player) {
            ui.show_victory_screen(&mut window);
            break;
        }

        if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
            view_mode = 1 - view_mode;
        }
    }
}
