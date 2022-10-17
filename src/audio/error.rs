//! # Audio error
//!
//! Audio error types

use rodio::{PlayError, StreamError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("play error: {0}")]
    Play(PlayError),
    #[error("stream error: {0}")]
    Stream(StreamError),
    #[error("failed to stop theme thread")]
    ThreadNotStopped,
}

impl From<PlayError> for AudioError {
    fn from(e: PlayError) -> Self {
        Self::Play(e)
    }
}

impl From<StreamError> for AudioError {
    fn from(e: StreamError) -> Self {
        Self::Stream(e)
    }
}
