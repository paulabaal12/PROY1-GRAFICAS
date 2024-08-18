mod player;
mod map;
mod renderer;
mod audio;
mod ui;
mod animated_sprite;
mod enemy;

use minifb::{Key, Window, WindowOptions};
use std::time::{Instant, Duration};

use player::Player;
use map::Map;
use renderer::Renderer;
use audio::AudioManager;
use ui::UI;
use enemy::Enemy;
use animated_sprite::AnimatedSprite;

const WIDTH: usize = 840;
const HEIGHT: usize = 580;
const TARGET_FPS: u32 = 15;

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
    animated_sprite: AnimatedSprite,
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
        let animated_sprite = AnimatedSprite::new("assets/sprite", 5, Duration::from_millis(200))
            .expect("Failed to create animated sprite");

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
            animated_sprite,
        }
    }

    fn handle_input(&mut self, window: &mut Window, dt: f64) {
        let move_speed = 2.0 * dt;

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
            self.player.rotate(dx * 0.001 * dt);
        }

        let now = Instant::now();
        if (window.is_key_down(Key::W) || window.is_key_down(Key::S) || window.is_key_down(Key::A) || window.is_key_down(Key::D)) 
            && now.duration_since(self.last_step_time) >= Duration::from_millis(500) {
            self.audio.play_footstep();
            self.last_step_time = now;
        }
    }

    fn update(&mut self, dt: f64) {
        self.enemy.update(&self.map, &self.player, dt);
        self.animated_sprite.update();
    }

    fn render(&mut self, window: &mut Window) {
        let mut buffer = vec![0; self.renderer.width * self.renderer.height];
        let mut z_buffer = vec![f64::MAX; self.renderer.width * self.renderer.height];
    
        self.renderer.render_3d(&self.map, &self.player, &mut buffer, &mut z_buffer);
        self.renderer.render_minimap(&self.map, &self.player, &self.enemy, &mut buffer);
        self.enemy.render(&mut buffer, self.renderer.width, self.renderer.height, &self.player, &z_buffer);
        self.ui.render_fps(self.fps, &mut buffer, self.renderer.width, self.renderer.height);
        self.animated_sprite.render(&mut buffer, self.renderer.width, self.renderer.height);
    
        window.update_with_buffer(&buffer, self.renderer.width, self.renderer.height).unwrap();
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


    fn show_welcome_screen(&mut self, window: &mut Window) {
        self.ui.show_welcome_screen(window);
        if window.is_key_down(Key::Space) {
            println!("Space pressed: Changing state to Playing");
            self.current_state = State::Playing;
            self.audio.play_background_music("assets/nobodynocrimets.mp3");
        }
    }


    fn show_victory_screen(&mut self, window: &mut Window) {
        self.ui.show_victory_screen(window);
        if !self.victory_sound_played {
            self.audio.play_victory();
            self.victory_sound_played = true;
        }
        if window.is_key_down(Key::Space) {
            self.reset_game();
        }
    }

    fn show_game_over_screen(&mut self, window: &mut Window) {
        self.ui.show_game_over_screen(window);
        if !self.game_over_sound_played {
            self.audio.play_game_over();
            self.game_over_sound_played = true;
        }
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

fn main() {
    let frame_duration = Duration::from_secs_f64(1.0 / TARGET_FPS as f64);

    let mut window = Window::new(
        "Ray Caster",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| { panic!("{}", e); });

    window.set_cursor_visibility(false);

    let mut game_state = GameState::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();

        match game_state.current_state {
            State::Welcome => game_state.show_welcome_screen(&mut window),
            State::Playing => game_state.play(&mut window),
            State::Victory => game_state.show_victory_screen(&mut window),
            State::GameOver => game_state.show_game_over_screen(&mut window),
        }

        let frame_end = Instant::now();
        let frame_time = frame_end.duration_since(frame_start);
        if frame_time < frame_duration {
            std::thread::sleep(frame_duration - frame_time);
        }
    }
}
