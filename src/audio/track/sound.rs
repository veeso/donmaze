//! # Sound
//!
//! Sound

use super::Track;

/// Donmaze sound type
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Sound {
    ArmorEquipped,
    DrinkPotion,
    EnemyApproaching,
    EnemyAttack,
    EnemyScream,
    ItemCollected,
    LeaveMaze,
    PlayerAttack,
    PlayerDead,
    Rush,
    Sonar,
    Sleep,
    Steps,
    WakeUp,
}

impl Sound {
    pub fn track(self) -> Track {
        todo!()
    }
}
