mod map;
mod player;
mod raycaster;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use map::Map;
use player::Player;
use raycaster::RayCaster;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Ray Caster", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let map = Map::new("maze.txt");
    let player = Player::new(1.5, 1.5);
    let mut raycaster = RayCaster::new(map, player)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut render_mode = 0; // 0 for 3D, 1 for 2D

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    raycaster.rotate_player(-0.1);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    raycaster.rotate_player(0.1);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    raycaster.move_player_forward(0.1);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    raycaster.move_player_backward(0.1);
                },
                Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
                    render_mode = 1 - render_mode; // Toggle between 0 and 1
                },
                _ => {}
            }
        }

        if render_mode == 0 {
            raycaster.render_3d(&mut canvas, WIDTH, HEIGHT);
        } else {
            raycaster.render_2d(&mut canvas, WIDTH, HEIGHT);
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}