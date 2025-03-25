//! # Track
//!
//! Audio track

mod note;
mod sound;
mod theme;

use std::time::Duration;

use note::Note;
use rodio::source::{Amplify, SineWave, Source, TakeDuration};
pub use sound::Sound;
pub use theme::Theme;

type Tone = Amplify<TakeDuration<SineWave>>;

/// Raw Audio track
#[derive(Default, Clone)]
pub struct Track {
    pub tones: Vec<Tone>,
}

impl Track {
    /// Push tone to track
    pub fn tone(mut self, freq: f32, millis: u64, amplify: f32) -> Self {
        self.tones.push(
            SineWave::new(freq)
                .take_duration(Duration::from_millis(millis))
                .amplify(amplify),
        );
        self
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_make_track() {
        assert_eq!(
            Track::default()
                .tone(32.0, 10, 1.0)
                .tone(64.0, 20, 1.0)
                .tones
                .len(),
            2
        );
    }
}
