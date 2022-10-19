//! # Inventory
//!
//! Player's inventory

use std::collections::{hash_map::Iter, HashMap};

use super::entity::Item;

/// Current player's inventory stores all the items collected by the player
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Inventory {
    /// Association between item type and quantity
    items: HashMap<Item, u8>,
}

impl Inventory {
    /// Add item to inventory.
    /// If not in inventory, item is set to 1 as qty, otherwise qty is increased
    pub fn add(&mut self, item: Item) {
        let qty = self.items.get(&item).map(|x| x + 1).unwrap_or(1);
        self.items.insert(item, qty);
    }

    /// Consume item in inventory.
    /// Panics if not in inventory.
    /// If quantity is 1; item is deleted by inventory; otherwise is just decreased
    pub fn consume(&mut self, item: Item) {
        match *self.items.get(&item).unwrap() {
            1 => {
                self.items.remove(&item);
            }
            qty => {
                self.items.insert(item, qty - 1);
            }
        }
    }

    /// Returns whether inventory contains item
    pub fn has(&self, item: Item) -> bool {
        self.items.contains_key(&item)
    }

    /// Get an iterator over inventory
    pub fn items(&self) -> Iter<'_, Item, u8> {
        self.items.iter()
    }
}
