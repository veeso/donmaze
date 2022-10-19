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
    Error,
    Input,
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
        match self {
            Sound::ArmorEquipped => todo!(),
            Sound::DrinkPotion => todo!(),
            Sound::EnemyApproaching => todo!(),
            Sound::EnemyAttack => todo!(),
            Sound::EnemyScream => todo!(),
            Sound::Error => Self::error(),
            Sound::Input => Self::input(),
            Sound::ItemCollected => todo!(),
            Sound::LeaveMaze => todo!(),
            Sound::PlayerAttack => todo!(),
            Sound::PlayerDead => todo!(),
            Sound::Rush => todo!(),
            Sound::Sonar => todo!(),
            Sound::Sleep => todo!(),
            Sound::Steps => todo!(),
            Sound::WakeUp => todo!(),
        }
    }

    fn error() -> Track {
        Track::default()
            .tone(150.0, 25, 1.0)
            .tone(0.0, 20, 1.0)
            .tone(200.0, 50, 1.0)
    }

    fn input() -> Track {
        Track::default().tone(4000.0, 15, 0.2)
    }
}
