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
    pub fn name(&self) -> &str {
        match self {
            Self::DonMaze => "Don Maze",
            Self::Daemon(_) => "Daemon",
            Self::Shadow(_) => "Shadow",
        }
    }

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

    /// Accuracy 1-100 for enemy
    pub fn accuracy(&self) -> u8 {
        match self {
            Self::Daemon(_) => 85,
            Self::DonMaze => 50,
            Self::Shadow(_) => 95,
        }
    }

    /// Return base attack
    pub fn base_attack(&self) -> u8 {
        match self {
            Self::Daemon(_) => 3,
            Self::DonMaze => 255,
            Self::Shadow(_) => 2,
        }
    }
}

/// A daemon is an enemy which deals 1HP damage to player.
/// HP is between 2-10
/// Base attack: 3
/// Accuracy: 85
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
/// Accuracy: 95
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Shadow {
    health: Hp,
}

impl Shadow {
    pub fn new(health: Hp) -> Self {
        Self { health }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_enemy_name() {
        assert_eq!(Enemy::Daemon(Daemon { health: 2 }).name(), "Daemon");
        assert_eq!(Enemy::DonMaze.name(), "Don Maze");
        assert_eq!(Enemy::Shadow(Shadow { health: 3 }).name(), "Shadow");
    }

    #[test]
    fn should_get_health() {
        assert_eq!(Enemy::Daemon(Daemon { health: 2 }).health(), 2);
        assert_eq!(Enemy::DonMaze.health(), 255);
        assert_eq!(Enemy::Shadow(Shadow { health: 3 }).health(), 3);
    }

    #[test]
    fn should_damage_daemon() {
        let mut daemon = Enemy::Daemon(Daemon { health: 2 });
        daemon.damage(1);
        assert_eq!(daemon.health(), 1);
        daemon.damage(4);
        assert_eq!(daemon.health(), 0);
    }

    #[test]
    fn should_damage_shadow() {
        let mut shadow = Enemy::Shadow(Shadow { health: 2 });
        shadow.damage(1);
        assert_eq!(shadow.health(), 1);
        shadow.damage(4);
        assert_eq!(shadow.health(), 0);
    }

    #[test]
    fn should_not_damage_don_maze() {
        let mut don_maze = Enemy::DonMaze;
        don_maze.damage(255);
        assert_eq!(don_maze.health(), 255);
    }
}
