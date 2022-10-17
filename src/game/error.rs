//! # Game error
//!
//! Game error types

use crate::{audio::AudioError, ui::UiError};
use thiserror::Error;

/// Game error
#[derive(Debug, Error)]
pub enum Error {
    #[error("audio error: {0}")]
    Audio(AudioError),
    #[error("ui error: {0}")]
    Ui(UiError),
    #[error("saved game has an incompatible version; you need to start a new game")]
    UnsupportedVersion,
}

impl From<AudioError> for Error {
    fn from(e: AudioError) -> Self {
        Self::Audio(e)
    }
}

impl From<UiError> for Error {
    fn from(e: UiError) -> Self {
        Self::Ui(e)
    }
}
