//! # Track
//!
//! Audio track

mod sound;
mod theme;

pub use sound::Sound;
pub use theme::Theme;

use rodio::source::{Amplify, SineWave, Source, TakeDuration};
use std::time::Duration;

type Tone = Amplify<TakeDuration<SineWave>>;

/// Raw Audio track
#[derive(Default, Clone)]
pub struct Track {
    pub duration: Duration,
    pub tones: Vec<Tone>,
}

impl Track {
    /// Push tone to track
    pub fn tone(mut self, freq: f32, millis: u64, amplify: f32) -> Self {
        self.duration += Duration::from_millis(millis);
        self.tones.push(
            SineWave::new(freq)
                .take_duration(Duration::from_millis(millis))
                .amplify(amplify),
        );
        self
    }

    /// Return track length
    pub fn duration(&self) -> Duration {
        self.duration
    }
}
