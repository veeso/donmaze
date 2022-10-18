//! # Enemy

use crate::game::Hp;

/// Enemies in the maze
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(tag = "type")]
pub enum Enemy {
    /// The boss; OHKO
    DonMaze,
    Daemon(Daemon),
    Shadow(Shadow),
}

/// A daemon is an enemy which deals 1HP damage to player.
/// HP is between 2-10
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Daemon {
    health: Hp,
}

impl Daemon {
    pub fn new(health: Hp) -> Self {
        Self { health }
    }
}

/// A shadow is an enemy which deals 1HP damage to player.
/// HP is between 2-5
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Shadow {
    health: Hp,
}

impl Shadow {
    pub fn new(health: Hp) -> Self {
        Self { health }
    }
}
