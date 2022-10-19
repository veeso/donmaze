//! # Load game
//!
//! Load game view components

mod games;
mod metadata;
mod popup;

use std::path::PathBuf;

use super::Msg;
pub use games::Games;
pub use metadata::{LastTurn, Seed, Turn};
pub use popup::ErrorPopup;

/// Menu ids
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LoadGameId {
    Games,
    ErrorPopup,
    LastTurn,
    Seed,
    Turn,
}

/// Messages related to main menu
#[derive(PartialEq, Eq)]
pub enum LoadGameMsg {
    CloseErrorPopup,
    GameChanged(PathBuf),
    LoadGame(PathBuf),
    GoToMenu,
}
