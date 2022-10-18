///! # Game
///
/// Main game core engine and logics
use crate::audio::{AudioEngine, Sound};
use crate::gfx::Render;
use crate::ui::Ui;

mod entity;
mod error;
mod inventory;
mod maze;
mod session;

pub use error::Error as GameError;
use session::Session;

pub type GameResult<T> = Result<T, GameError>;
/// Health points
pub type Hp = u8;

/// Game runtime
pub struct Runtime {
    audio: Option<AudioEngine>,
    render: Render,
    session: Option<Session>,
    ui: Ui,
}

impl Runtime {
    /// Setup game runtime
    pub fn setup() -> GameResult<Self> {
        todo!()
    }

    /// Run game
    pub fn run(mut self) -> GameResult<()> {
        todo!()
        // TODO: check version compatible
    }

    /// Play sound
    fn play_sound(&mut self, sound: Sound) {
        if let Some(audio) = self.audio.as_mut() {
            audio.play(sound.track());
        }
    }
}
