//! # Sound
//!
//! Sound

use super::{Note, Track};

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
    GameSaved,
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
            Sound::ArmorEquipped => Self::armor_equipped(),
            Sound::DrinkPotion => Self::drink_potion(),
            Sound::EnemyApproaching => Self::enemy_approaching(),
            Sound::EnemyAttack => Self::enemy_attack(),
            Sound::EnemyScream => Self::enemy_scream(),
            Sound::Error => Self::error(),
            Sound::GameSaved => Self::game_saved(),
            Sound::Input => Self::input(),
            Sound::ItemCollected => Self::item_collected(),
            Sound::LeaveMaze => Self::leave_maze(),
            Sound::PlayerAttack => Self::player_attack(),
            Sound::PlayerDead => Self::player_dead(),
            Sound::Rush => Self::rush(),
            Sound::Sonar => Self::sonar(),
            Sound::Sleep => Self::sleep(),
            Sound::Steps => Self::steps(),
            Sound::WakeUp => Self::wake_up(),
        }
    }

    fn armor_equipped() -> Track {
        Track::default()
            .tone(2000.0, 25, 0.2)
            .tone(0.0, 20, 1.0)
            .tone(3000.0, 50, 0.2)
    }

    fn drink_potion() -> Track {
        Track::default()
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
            .tone(0.0, 400, 1.0)
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
            .tone(0.0, 400, 1.0)
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
            .tone(0.0, 400, 1.0)
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
            .tone(0.0, 400, 1.0)
            .tone(80.0, 200, 1.0)
            .tone(100.0, 125, 1.0)
    }

    fn enemy_approaching() -> Track {
        Track::default()
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 600, 0.2)
            .tone(Note::Mi.freq(2), 500, 0.8)
            .tone(Note::Mi.freq(2), 500, 0.8)
            .tone(Note::Mi.freq(2), 500, 0.8)
            .tone(Note::Do.freq(2), 1000, 0.8)
    }

    fn enemy_attack() -> Track {
        Track::default()
            .tone(50.0, 300, 1.0)
            .tone(70.0, 100, 1.0)
            .tone(100.0, 250, 1.0)
    }

    fn enemy_scream() -> Track {
        Track::default()
            .tone(100.0, 120, 1.0)
            .tone(120.0, 120, 1.0)
            .tone(140.0, 500, 1.0)
    }

    fn error() -> Track {
        Track::default()
            .tone(150.0, 25, 1.0)
            .tone(0.0, 20, 1.0)
            .tone(200.0, 50, 1.0)
    }

    fn game_saved() -> Track {
        Track::default().tone(4000.0, 40, 0.2)
    }

    fn input() -> Track {
        Track::default().tone(4000.0, 15, 0.2)
    }

    fn item_collected() -> Track {
        Track::default()
            .tone(Note::La.freq(5), 200, 0.2)
            .tone(Note::As.freq(5), 200, 0.2)
            .tone(Note::Si.freq(5), 200, 0.2)
            .tone(Note::Do.freq(6), 500, 0.2)
    }

    fn leave_maze() -> Track {
        Track::default()
            .tone(Note::Mi.freq(4), 150, 1.0)
            .tone(Note::Mi.freq(4), 150, 1.0)
            .tone(Note::Mi.freq(4), 150, 1.0)
            .tone(Note::La.freq(4), 750, 1.0)
    }

    fn player_attack() -> Track {
        Track::default().tone(120.0, 130, 1.0).tone(110.0, 80, 1.0)
    }

    fn player_dead() -> Track {
        Track::default()
            .tone(Note::Ds.freq(5), 200, 0.3)
            .tone(Note::D.freq(5), 200, 0.3)
            .tone(Note::Ds.freq(5), 200, 0.3)
            .tone(Note::D.freq(5), 200, 0.3)
            .tone(Note::Ds.freq(5), 400, 0.3)
            .tone(Note::D.freq(5), 400, 0.3)
            .tone(Note::Ds.freq(5), 700, 0.3)
            .tone(Note::D.freq(5), 700, 0.3)
            .tone(Note::C.freq(4), 700, 1.0)
    }

    fn rush() -> Track {
        Track::default()
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 100, 0.2)
    }

    fn sleep() -> Track {
        Track::default()
            .tone(Note::Mi.freq(1), 400, 1.0)
            .tone(Note::Mi.freq(1), 400, 1.0)
            .tone(Note::Sol.freq(1), 600, 1.0)
            .tone(0.0, 600, 1.0)
            .tone(Note::Mi.freq(1), 400, 1.0)
            .tone(Note::Mi.freq(1), 400, 1.0)
            .tone(Note::Sol.freq(1), 600, 1.0)
    }

    fn sonar() -> Track {
        Track::default()
            .tone(Note::Mi.freq(7), 200, 0.01)
            .tone(0.0, 300, 1.0)
            .tone(Note::Mi.freq(7), 200, 0.01)
            .tone(0.0, 300, 1.0)
            .tone(Note::Mi.freq(7), 200, 0.01)
            .tone(0.0, 300, 1.0)
            .tone(Note::Mi.freq(7), 1000, 0.01)
    }

    fn steps() -> Track {
        Track::default()
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
            .tone(10.0, 200, 4.0)
            .tone(0.0, 300, 0.2)
    }

    fn wake_up() -> Track {
        Track::default()
            .tone(Note::G.freq(5), 700, 0.3)
            .tone(Note::B.freq(5), 700, 0.3)
            .tone(Note::A.freq(5), 700, 0.3)
            .tone(Note::D.freq(5), 1000, 0.3)
            .tone(Note::D.freq(5), 700, 0.3)
            .tone(Note::A.freq(5), 700, 0.3)
            .tone(Note::B.freq(5), 700, 0.3)
            .tone(Note::G.freq(5), 1000, 0.3)
    }
}
