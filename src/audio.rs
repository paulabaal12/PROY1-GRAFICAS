use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct AudioManager {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioManager {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioManager { _stream: stream, sink }
    }

    pub fn play_background_music(&self, file_path: &str) {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.sink.set_volume(0.5);
        self.sink.play();
    }

    pub fn play_footstep(&self) {
        let file = BufReader::new(File::open("assets/footsteps.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.sink.set_volume(0.5);
        self.sink.play();
    }

    pub fn play_victory(&self) {
        let file = BufReader::new(File::open("assets/victory.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.sink.set_volume(0.5);
        self.sink.play();
    }

    pub fn play_game_over(&self) {
        let file = BufReader::new(File::open("assets/gameover.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.sink.set_volume(0.5);
        self.sink.play();
    }
}
