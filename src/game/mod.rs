///! # Game
///
/// Main game core engine and logics
use crate::audio::AudioEngine;
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

/// Game runtime
pub struct Runtime {
    audio: AudioEngine,
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
    }
}
