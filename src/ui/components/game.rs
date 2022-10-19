//! # Game
//!
//! Game components

mod actions;
mod canvas;
mod hp;
mod inventory;
mod label;
mod messages;
mod popup;

pub use actions::AvailableActions;
pub use canvas::Canvas;
pub use hp::{EnemyHp, PlayerHp};
pub use inventory::Inventory;
pub use label::EnemyName;
pub use messages::Messages;
pub use popup::{ErrorPopup, GameOverPopup, QuitPopup, SaveFileNamePopup};

use crate::game::{entity::Item, session::Action};

use super::Msg;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameId {
    AvailableActions,
    Canvas,
    EnemyHp,
    EnemyName,
    ErrorPopup,
    GameOverPopup,
    Inventory,
    Messages,
    PlayerHp,
    SaveFileNamePopup,
    QuitPopup,
}

/// Messages related to game
#[derive(PartialEq, Eq)]
pub enum GameMsg {
    ActionSelected(Action),
    CloseErrorPopup,
    CloseInventory,
    CloseQuitPopup,
    CloseSaveFileName,
    GameOver,
    /// If true, save game
    Quit(bool),
    SaveGame(String),
    ShowInventory,
    ShowSaveFileName,
    ShowQuitPopup,
    UseItem(Item),
}
