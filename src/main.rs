mod player;
mod map;
mod renderer;
mod audio;
mod ui;
mod enemy;

use minifb::{Key, Window, WindowOptions};
use std::time::{Instant, Duration};

use player::Player;
use map::Map;
use renderer::Renderer;
use audio::AudioManager;
use ui::UI;
use enemy::Enemy;

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

    let mut game_state = GameState::new();

    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        match game_state.current_state {
            State::Welcome => game_state.show_welcome_screen(&mut window),
            State::Playing => game_state.play(&mut window),
            State::Victory => game_state.show_victory_screen(&mut window),
            State::GameOver => game_state.show_game_over_screen(&mut window),
        }
        window.update();
    }
}

pub struct GameState {
    map: Map,
    player: Player,
    enemy: Enemy,
    renderer: Renderer,
    audio: AudioManager,
    ui: UI,
    current_state: State,
    last_frame_time: Instant,
    fps_counter: u32,
    fps: u32,
    last_step_time: Instant,
    victory_sound_played: bool,
    game_over_sound_played: bool,
}

enum State {
    Welcome,
    Playing,
    Victory,
    GameOver,
}

impl GameState {
    pub fn new() -> Self {
        let map = Map::load("assets/maze.txt");
        let player = Player::new(&map);
        let enemy = Enemy::new(&map);
        let renderer = Renderer::new(WIDTH, HEIGHT);
        let audio = AudioManager::new();
        let ui = UI::new();

        GameState {
            map,
            player,
            enemy,
            renderer,
            audio,
            ui,
            current_state: State::Welcome,
            last_frame_time: Instant::now(),
            fps_counter: 0,
            fps: 0,
            last_step_time: Instant::now(),
            victory_sound_played: false,
            game_over_sound_played: false,
        }
    }

  
    fn handle_input(&mut self, window: &mut Window, dt: f64) {
        let move_speed = 2.0 * dt; // Ajusta este valor para cambiar la velocidad de movimiento
        let rotation_speed = 2.0 * dt; // Ajusta este valor para cambiar la velocidad de rotación
    
        if window.is_key_down(Key::W) {
            self.player.move_forward(&self.map, move_speed);
        }
        if window.is_key_down(Key::S) {
            self.player.move_backward(&self.map, move_speed);
        }
        if window.is_key_down(Key::A) {
            self.player.strafe_left(&self.map, move_speed);
        }
        if window.is_key_down(Key::D) {
            self.player.strafe_right(&self.map, move_speed);
        }
    
        if let Some((x, _)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            let center_x = (self.renderer.width / 2) as f64;
            let dx = x as f64 - center_x;
            self.player.rotate(dx * rotation_speed);
        }
    
        self.player.update(dt);
    
        let now = Instant::now();
        if (window.is_key_down(Key::W) || window.is_key_down(Key::S) || window.is_key_down(Key::A) || window.is_key_down(Key::D)) 
            && now.duration_since(self.last_step_time) >= Duration::from_millis(500) {
            self.audio.play_footstep();
            self.last_step_time = now;
        }
    }

    fn update(&mut self, dt: f64) {
        self.enemy.update(&self.map, &self.player, dt);
    }
    
    fn render(&self, window: &mut Window) {
        let mut buffer = self.renderer.render_3d(&self.map, &self.player);
        self.renderer.render_minimap(&self.map, &self.player, &self.enemy, &mut buffer);
        self.enemy.render(&mut buffer, self.renderer.width, self.renderer.height, &self.player);
        self.ui.render_fps(self.fps, &mut buffer, self.renderer.width);
        window.update_with_buffer(&buffer, self.renderer.width, self.renderer.height).unwrap();
    }
   
    fn show_welcome_screen(&mut self, window: &mut Window) {
        self.ui.show_welcome_screen(window);
        if window.is_key_down(Key::Space) {
            self.current_state = State::Playing;
            self.audio.play_background_music("assets/nobodynocrimets.mp3");
        }
    }

    fn play(&mut self, window: &mut Window) {
        let frame_start = Instant::now();
        let dt = frame_start.duration_since(self.last_frame_time).as_secs_f64();
    
        self.handle_input(window, dt);
        self.update(dt);
        self.render(window);
    
        self.fps_counter += 1;
        if frame_start.duration_since(self.last_frame_time) >= Duration::from_secs(1) {
            self.fps = self.fps_counter;
            self.fps_counter = 0;
            self.last_frame_time = frame_start;
        }
    
        if self.map.is_player_at_goal(&self.player) {
            self.current_state = State::Victory;
        } else if self.enemy.has_caught_player(&self.player) {
            self.current_state = State::GameOver;
        }
    
        self.last_frame_time = frame_start;
    }
    
    fn show_victory_screen(&mut self, window: &mut Window) {
        if !self.victory_sound_played {
            self.audio.play_victory();
            self.victory_sound_played = true;
        }
        self.ui.show_victory_screen(window);
        if window.is_key_down(Key::Space) {
            self.reset_game();
        }
    }
    
    fn show_game_over_screen(&mut self, window: &mut Window) {
        if !self.game_over_sound_played {
            self.audio.play_game_over();
            self.game_over_sound_played = true;
        }
        self.ui.show_game_over_screen(window);
        if window.is_key_down(Key::Space) {
            self.reset_game();
        }
    }


    fn reset_game(&mut self) {
        self.player = Player::new(&self.map);
        self.enemy = Enemy::new(&self.map);
        self.current_state = State::Playing;
        self.victory_sound_played = false;
        self.game_over_sound_played = false;
    }
}

