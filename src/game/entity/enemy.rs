//! # Enemy

use crate::game::Hp;

/// Enemies in the maze
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(tag = "type")]
pub enum Enemy {
    /// The boss; OHKO
    DonMaze,
    Daemon(Daemon),
    Shadow(Shadow),
}

impl Enemy {
    /// Get enemy health
    pub fn health(&self) -> Hp {
        match self {
            Self::Daemon(Daemon { health }) => *health,
            Self::DonMaze => 255,
            Self::Shadow(Shadow { health }) => *health,
        }
    }

    /// Inflict damage to enemy
    pub fn damage(&mut self, hp: Hp) {
        match self {
            Self::Daemon(daemon) => {
                daemon.health = daemon.health.saturating_sub(hp);
            }
            Self::Shadow(shadow) => {
                shadow.health = shadow.health.saturating_sub(hp);
            }
            Self::DonMaze => {}
        }
    }
}

/// A daemon is an enemy which deals 1HP damage to player.
/// HP is between 2-10
/// Base attack: 3
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
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
/// Base attack: 2 (crit: 3)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Shadow {
    health: Hp,
}

impl Shadow {
    pub fn new(health: Hp) -> Self {
        Self { health }
    }
}
