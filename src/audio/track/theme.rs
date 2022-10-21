//! # Theme
//!
//! Donmaze theme

use super::{Note, Track};

/// Donmaze theme type
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Theme {
    Fight,
    GameOver,
    Maze,
    Menu,
    Victory,
    None,
}

impl Theme {
    pub fn track(self) -> Track {
        match self {
            Theme::Fight => Self::fight(),
            Theme::GameOver => Self::game_over(),
            Theme::Maze => Self::maze(),
            Theme::Menu => Self::menu(),
            Theme::Victory => Self::victory(),
            Theme::None => Track::default(),
        }
    }

    fn fight() -> Track {
        Track::default()
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::D.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::D.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::Cs.freq(2), 300, 4.0)
            .tone(Note::Ds.freq(2), 300, 4.0)
            .tone(Note::E.freq(2), 600, 4.0)
    }

    fn game_over() -> Track {
        Track::default()
            .tone(Note::E.freq(4), 500, 0.8)
            .tone(Note::D.freq(4), 500, 0.8)
            .tone(Note::Cs.freq(4), 2000, 0.8)
            .tone(Note::As.freq(3), 500, 2.0)
            .tone(Note::Gs.freq(3), 500, 2.0)
            .tone(Note::E.freq(3), 2000, 3.0)
            .tone(Note::F.freq(3), 500, 2.0)
            .tone(Note::Gs.freq(3), 1200, 2.0)
            .tone(Note::C.freq(4), 1000, 0.8)
    }

    fn maze() -> Track {
        Track::default()
            .tone(51.9, 2600, 4.0)
            .tone(116.6, 2600, 3.2)
            .tone(123.2, 2600, 3.0)
            .tone(116.6, 2600, 3.0)
    }

    fn menu() -> Track {
        Track::default()
            .tone(50.0, 300, 1.5)
            .tone(70.0, 300, 2.5)
            .tone(60.0, 300, 3.5)
            .tone(30.0, 300, 4.5)
            .tone(40.0, 300, 5.5)
            .tone(30.0, 300, 4.5)
            .tone(60.0, 300, 3.5)
            .tone(50.0, 300, 1.5)
            .tone(70.0, 300, 2.5)
    }

    fn victory() -> Track {
        Track::default()
            .tone(Note::E.freq(4), 500, 0.4)
            .tone(Note::E.freq(4), 500, 0.4)
            .tone(Note::F.freq(4), 500, 0.4)
            .tone(Note::G.freq(4), 500, 0.4)
            .tone(Note::Gs.freq(4), 500, 0.4)
            .tone(Note::F.freq(4), 500, 0.4)
            .tone(Note::E.freq(4), 500, 0.4)
            .tone(Note::E.freq(4), 500, 0.4)
            .tone(Note::F.freq(4), 500, 0.4)
            .tone(Note::Gs.freq(4), 500, 0.4)
            .tone(Note::Gs.freq(4), 500, 0.4)
            .tone(Note::G.freq(4), 500, 0.4)
            .tone(Note::F.freq(4), 500, 0.4)
    }
}
