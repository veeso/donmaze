//! # Enemy

/// Enemies in the maze
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(tag = "type")]
pub enum Enemy {
    /// The boss; OHKO
    DonMaze,
    Daemon(Daemon),
}

/// A daemon is an enemy which deals 1HP damage to player.
/// HP is between 2-10
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Daemon {
    health: u8,
}

impl Daemon {
    pub fn new(health: u8) -> Self {
        Self { health }
    }
}
