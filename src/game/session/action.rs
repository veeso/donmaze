//! # Action
//!
//! The action defines what the player performs in a turn

use crate::game::entity::Item;

/// Defines the action performed by the player in a turn
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    /// Action usable only in exploration
    Explore(ExploreAction),
    /// Action usable only in fight
    Fight(FightAction),
    /// Use item
    UseItem(Item),
    /// Game has been saved; consume turn
    SaveGame,
    /// Sleep, don't do anything. Can be used only when state is `Sleep`
    Sleep,
    /// Ends game
    Die,
}

/// Defines the action which can be performed while state is `Explore`
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExploreAction {
    /// Change room to provided node
    ChangeRoom(u32),
    /// Collect the item in the room
    CollectItem,
    /// Leave maze and win
    LeaveMaze,
    /// Go to previous room
    GoToPreviousRoom,
}

/// Defines the action which can be performed while state is `Explore`
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FightAction {
    /// Fight enemy
    Fight,
    /// Try to escape from fight
    Escape,
}
