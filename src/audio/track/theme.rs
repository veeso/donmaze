//! # Theme
//!
//! Donmaze theme

use super::Track;

/// Donmaze theme type
#[derive(Debug, Eq, PartialEq)]
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
            Theme::Fight => todo!(),
            Theme::GameOver => todo!(),
            Theme::Maze => todo!(),
            Self::Menu => Self::menu(),
            Theme::Victory => todo!(),
            Theme::None => Track::default(),
        }
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
}
