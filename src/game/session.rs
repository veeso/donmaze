//! # Session
//!
//! A game session. Contains all the current states for the game

use super::entity::Player;

/// The session contains all the game states.
/// It must be serializable since it is used to save and load games
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    player: Player,
}
