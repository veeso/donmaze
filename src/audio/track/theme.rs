//! # Theme
//!
//! Donmaze theme

use super::Track;

/// Donmaze theme type
#[derive(Debug, Eq, PartialEq)]
pub enum Theme {
    Fight,
    GameOver,
    Maze,
    Menu,
    Victory,
    None,
}

impl Theme {
    pub fn track(self) -> Track {
        todo!()
    }
}
