//! # Error
//!
//! Ui error

use thiserror::Error;
use tuirealm::{terminal::TerminalError, ApplicationError};

/// Ui error
#[derive(Debug, Error)]
pub enum UiError {
    #[error("application error: {0}")]
    Application(ApplicationError),
    #[error("terminal error: {0}")]
    Terminal(TerminalError),
}

impl From<ApplicationError> for UiError {
    fn from(e: ApplicationError) -> Self {
        Self::Application(e)
    }
}

impl From<TerminalError> for UiError {
    fn from(e: TerminalError) -> Self {
        Self::Terminal(e)
    }
}
