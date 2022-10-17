//! # Inventory
//!
//! Player's inventory

use std::collections::HashMap;

use super::entity::Item;

/// Current player's inventory stores all the items collected by the player
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Inventory {
    /// Association between item type and quantity
    items: HashMap<Item, u8>,
}
