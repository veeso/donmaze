//! # Sound
//!
//! Sound

use super::Track;

/// Donmaze sound type
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Sound {
    DrinkPotion,
    PlayerDead,
    Sleep,
    Steps,
    WakeUp,
}

impl Sound {
    pub fn track(self) -> Track {
        todo!()
    }
}
