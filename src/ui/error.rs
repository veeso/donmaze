//! # Error
//!
//! Ui error

use thiserror::Error;
use tuirealm::terminal::TerminalError;
use tuirealm::ApplicationError;

/// Ui error
#[derive(Debug, Error)]
pub enum UiError {
    #[error("failed to get terminal size")]
    FailedToGetSize,
    #[error("application error: {0}")]
    Application(ApplicationError),
    #[error("io error: {0}")]
    Io(std::io::Error),
    #[error("terminal error: {0}")]
    Terminal(TerminalError),
}

impl From<ApplicationError> for UiError {
    fn from(e: ApplicationError) -> Self {
        Self::Application(e)
    }
}

impl From<std::io::Error> for UiError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<TerminalError> for UiError {
    fn from(e: TerminalError) -> Self {
        Self::Terminal(e)
    }
}
