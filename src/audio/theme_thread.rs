//! # Theme thread

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use rodio::{OutputStream, Sink};

use super::track::Track;
use super::Theme;

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
            self.sink.sleep_until_end();
        }
    }
}
