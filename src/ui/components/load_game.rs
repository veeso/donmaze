//! # Load game
//!
//! Load game view components

mod games;
mod metadata;
mod popup;

use std::path::PathBuf;

pub use games::Games;
pub use metadata::Metadata;
pub use popup::ErrorPopup;

use super::Msg;

/// Menu ids
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LoadGameId {
    Games,
    ErrorPopup,
    Metadata,
}

/// Messages related to main menu
#[derive(PartialEq, Eq)]
pub enum LoadGameMsg {
    CloseErrorPopup,
    GameChanged(PathBuf),
    LoadGame(PathBuf),
    GoToMenu,
}
