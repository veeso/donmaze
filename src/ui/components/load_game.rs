//! # Load game
//!
//! Load game view components

mod games;
mod metadata;

use std::path::PathBuf;

use super::Msg;
pub use games::Games;
pub use metadata::{LastTurn, Seed, Turn};

/// Menu ids
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LoadGameId {
    Games,
    LastTurn,
    Seed,
    Turn,
}

/// Messages related to main menu
#[derive(PartialEq, Eq)]
pub enum LoadGameMsg {
    GameChanged(PathBuf),
    LoadGame(PathBuf),
    GoToMenu,
}
