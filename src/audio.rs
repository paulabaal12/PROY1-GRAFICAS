use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    sink: Sink,
}

impl AudioManager {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioManager { _stream: stream, stream_handle, sink }
    }

    pub fn play_background_music(&self, file_path: &str) {
        self.sink.stop(); // Stop any currently playing audio
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.sink.set_volume(0.5);
        self.sink.play();
    }

    pub fn play_footstep(&self) {
        let file = BufReader::new(File::open("assets/footsteps.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        let footstep_sink = Sink::try_new(&self.stream_handle).unwrap();
        footstep_sink.append(source);
        footstep_sink.set_volume(0.5);
        footstep_sink.play();
    }

    pub fn play_victory(&self) {
        self.sink.stop(); // Stop any currently playing audio
        let file = BufReader::new(File::open("assets/victory.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.sink.set_volume(0.5);
        self.sink.play();
    }

    pub fn play_game_over(&self) {
        self.sink.stop(); 
        let file = BufReader::new(File::open("assets/gameover.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.sink.set_volume(0.5);
        self.sink.play();
    }

    pub fn is_playing(&self) -> bool {
        !self.sink.empty()
    }
}

