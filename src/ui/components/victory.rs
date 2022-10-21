//! # Victory

use super::Msg;

mod buttons;
mod texts;

pub use buttons::GoToMenu;
pub use texts::{Stats, Title};

/// Victory ids
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum VictoryId {
    Title,
    Stats,
    GoToMenu,
}

/// Messages related to victory
#[derive(PartialEq, Eq)]
pub enum VictoryMsg {
    GoToMenu,
}
