//! # Theme thread

use super::track::Track;
use super::Theme;

use rodio::{OutputStream, Sink};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

pub struct ThemeThread {
    running: Arc<AtomicBool>,
    sink: Sink,
    _stream: OutputStream,
    theme: Track,
}

impl ThemeThread {
    pub fn new(running: Arc<AtomicBool>, theme: Theme) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            running,
            sink,
            _stream: stream,
            theme: theme.track(),
        }
    }

    pub fn run(self) {
        while self.running.load(Ordering::Relaxed) {
            for tone in self.theme.clone().tones {
                self.sink.append(tone);
            }
            thread::sleep(self.theme.duration());
        }
    }
}
