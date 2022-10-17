//! # Game
//!
//! Game components

mod canvas;

pub use canvas::Canvas;

use super::Msg;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameId {}

/// Messages related to game
#[derive(PartialEq, Eq)]
pub enum GameMsg {}
