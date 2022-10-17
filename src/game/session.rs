//! # Session
//!
//! A game session. Contains all the current states for the game

use super::{entity::Player, maze::Maze};

/// Game version (does not refer to the game itself, but to the engine, to track compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum Version {
    V010,
}

impl Version {
    /// Returns whether game version is compatible
    pub fn is_compatible(&self) -> bool {
        [Self::V010].contains(self)
    }
}

/// The session contains all the game states.
/// It must be serializable since it is used to save and load games
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    maze: Maze,
    player: Player,
    /// Game version; used to check whether this version loaded is compatible
    version: Version,
}
