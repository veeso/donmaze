//! # Game over

use super::Msg;

mod buttons;
mod texts;

pub use buttons::GoToMenu;
pub use texts::{Stats, Title};

/// game over ids
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameOverId {
    GoToMenu,
    Stats,
    Title,
}

/// Messages related to game over
#[derive(PartialEq, Eq)]
pub enum GameOverMsg {
    GoToMenu,
}
