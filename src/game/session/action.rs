//! # Action
//!
//! The action defines what the player performs in a turn

use crate::game::entity::Item;

/// Defines the action performed by the player in a turn
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Action usable in exploration
    Explore(ExploreAction),
    /// Action usable in fight
    Fight(FightAction),
    /// Sleep, don't do anything. Can be used only when state is `Sleep`
    Sleep,
    /// Ends game
    Die,
}

/// Defines the action which can be performed while state is `Explore`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExploreAction {
    /// Go to previous room
    GoToPreviousRoom,
    /// Change room to provided node
    ChangeRoom(u32),
    /// Use item
    UseItem(Item),
}

/// Defines the action which can be performed while state is `Explore`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FightAction {
    /// Fight enemy
    Fight,
    /// Try to escape from fight
    Escape,
    /// Use item
    UseItem(Item),
}
