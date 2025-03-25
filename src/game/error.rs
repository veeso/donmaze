//! # Game error
//!
//! Game error types

use thiserror::Error;

use crate::audio::AudioError;
use crate::ui::UiError;
use crate::utils::saved_games::SavedGameError;

/// Game error
#[derive(Debug, Error)]
pub enum Error {
    #[error("audio error: {0}")]
    Audio(AudioError),
    #[error("game save error: {0}")]
    SaveGame(SavedGameError),
    #[error("ui error: {0}")]
    Ui(UiError),
}

impl From<AudioError> for Error {
    fn from(e: AudioError) -> Self {
        Self::Audio(e)
    }
}

impl From<SavedGameError> for Error {
    fn from(e: SavedGameError) -> Self {
        Self::SaveGame(e)
    }
}

impl From<UiError> for Error {
    fn from(e: UiError) -> Self {
        Self::Ui(e)
    }
}
