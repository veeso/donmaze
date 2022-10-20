//! # Audio
//!
//! Audio engine and resources

mod error;
mod theme_thread;
mod track;

pub use error::AudioError;
use theme_thread::ThemeThread;
use track::Track;
pub use track::{Sound, Theme};

use rodio::{OutputStream, Sink};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};

pub type AudioResult<T> = Result<T, AudioError>;

/// Donmaze audio engine
pub struct AudioEngine {
    sink: Sink,
    _stream: OutputStream,
    theme_running: Arc<AtomicBool>,
    theme_thread: Option<JoinHandle<()>>,
}

impl AudioEngine {
    /// Create a new Audio Engine, with provided theme
    pub fn new(theme: Theme) -> AudioResult<Self> {
        let theme_running = Arc::new(AtomicBool::new(false));
        debug!("creating audio stream");
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        debug!("audio stream OK");
        let mut engine = AudioEngine {
            sink,
            _stream: stream,
            theme_running,
            theme_thread: None,
        };
        debug!("playing theme {:?}", theme);
        engine.play_theme(theme)?;
        Ok(engine)
    }

    /// Stop current theme and start new theme
    pub fn play_theme(&mut self, theme: Theme) -> AudioResult<()> {
        self.stop_thread()?;
        if theme != Theme::None {
            debug!("thread stopped; starting thread");
            self.theme_running.store(true, Ordering::Relaxed);
            let thread_running = Arc::clone(&self.theme_running);
            let thread = thread::spawn(|| ThemeThread::new(thread_running, theme).run());
            debug!("theme thread started");
            self.theme_thread = Some(thread);
        } else {
            debug!("theme is `None`; not playing any theme");
        }
        Ok(())
    }

    /// Play track
    pub fn play(&self, track: Track) {
        for tone in track.tones {
            self.sink.append(tone);
        }
        self.sink.sleep_until_end();
    }

    // -- private

    fn stop_thread(&mut self) -> AudioResult<()> {
        debug!("setting running to false");
        self.theme_running.store(false, Ordering::Relaxed);
        if let Some(thread) = self.theme_thread.take() {
            debug!("waiting for thread to finish");
            thread.join().map_err(|_| AudioError::ThreadNotStopped)?;
        }
        Ok(())
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.stop_thread().expect("failed to stop theme thread");
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_init_audio_without_theme() {
        let audio = AudioEngine::new(Theme::None).unwrap();
        assert_eq!(audio.theme_running.load(Ordering::Relaxed), false);
        assert!(audio.theme_thread.is_none());
    }

    #[test]
    fn should_init_audio_with_theme() {
        let audio = AudioEngine::new(Theme::Menu).unwrap();
        assert_eq!(audio.theme_running.load(Ordering::Relaxed), true);
        assert!(audio.theme_thread.is_some());
    }

    #[test]
    fn should_play_sound() {
        let audio = AudioEngine::new(Theme::None).unwrap();
        audio.play(Sound::ArmorEquipped.track());
    }

    #[test]
    fn should_play_theme() {
        let mut audio = AudioEngine::new(Theme::None).unwrap();
        audio.play_theme(Theme::Menu).unwrap();
        assert_eq!(audio.theme_running.load(Ordering::Relaxed), true);
        assert!(audio.theme_thread.is_some());
    }
}
