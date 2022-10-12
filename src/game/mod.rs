///! # Game
///
/// Main game core engine and logics
use crate::audio::AudioEngine;

mod error;
pub use error::Error as GameError;

pub type GameResult<T> = Result<T, GameError>;

/// Game runtime
pub struct Runtime {
    audio: AudioEngine,
}

impl Runtime {
    pub fn setup() -> GameResult<Self> {
        todo!()
    }

    pub fn run(mut self) -> GameResult<()> {
        todo!()
    }
}
