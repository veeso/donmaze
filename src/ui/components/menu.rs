//! # Menu
//!
//! Menu components

mod buttons;
mod input;
mod title;

pub use buttons::{Exit, LoadGame, NewGame};
pub use input::Seed;
pub use title::Title;

use super::Msg;

/// Menu ids
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MenuId {
    Title,
    NewGame,
    LoadGame,
    Seed,
    Exit,
}

/// Messages related to main menu
#[derive(PartialEq, Eq)]
pub enum MenuMsg {
    ActiveNewGame,
    ActiveLoadGame,
    ActiveExit,
    ActiveSeed,
    NewGame,
    LoadGame,
    Quit,
}
