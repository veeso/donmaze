//! # Inventory
//!
//! Player's inventory

use std::collections::HashMap;

use super::entity::Item;

/// Current player's inventory stores all the items collected by the player
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Inventory {
    /// Association between item type and quantity
    items: HashMap<u32, u8>,
}

impl Inventory {
    /// Add item to inventory.
    /// If not in inventory, item is set to 1 as qty, otherwise qty is increased
    pub fn add(&mut self, item: Item) {
        let key = item.key();
        let qty = self.items.get(&key).map(|x| x + 1).unwrap_or(1);
        self.items.insert(key, qty);
    }

    /// Consume item in inventory.
    /// Panics if not in inventory.
    /// If quantity is 1; item is deleted by inventory; otherwise is just decreased
    pub fn consume(&mut self, item: Item) {
        let key = item.key();
        match *self.items.get(&key).unwrap() {
            1 => {
                self.items.remove(&key);
            }
            qty => {
                self.items.insert(key, qty - 1);
            }
        }
    }

    /// Returns whether inventory contains item
    pub fn has(&self, item: Item) -> bool {
        self.items.contains_key(&item.key())
    }

    /// Get an iterator over inventory
    pub fn items(&self) -> Vec<(Item, u8)> {
        let mut items = Vec::with_capacity(self.items.len());
        for (key, qty) in self.items.iter() {
            items.push((Item::from(*key), *qty));
        }
        items
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_make_inventory() {
        let mut inventory = Inventory::default();
        assert!(inventory.items.is_empty());
        assert_eq!(inventory.has(Item::AlchemyBook), false);
        inventory.add(Item::AlchemyBook);
        assert_eq!(inventory.has(Item::AlchemyBook), true);
        // consumable
        inventory.add(Item::Talisman);
        assert_eq!(*inventory.items.get(&Item::Talisman.key()).unwrap(), 1);
        inventory.add(Item::Talisman);
        assert_eq!(*inventory.items.get(&Item::Talisman.key()).unwrap(), 2);
        inventory.consume(Item::Talisman);
        assert_eq!(*inventory.items.get(&Item::Talisman.key()).unwrap(), 1);
        inventory.consume(Item::Talisman);
        assert!(inventory.items.get(&Item::Talisman.key()).is_none());
    }

    #[test]
    fn should_iter_inventory() {
        let mut inventory = Inventory::default();
        inventory.add(Item::Talisman);
        inventory.add(Item::Talisman);
        inventory.add(Item::MazeKey);
        assert_eq!(inventory.items().len(), 2);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_consuming_unexisting_inventory() {
        let mut inventory = Inventory::default();
        inventory.consume(Item::Armor);
    }

    #[test]
    fn should_serialize() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct Test {
            inventory: Inventory,
        }
        let mut inventory = Inventory::default();
        inventory.add(Item::AlchemyBook);
        inventory.add(Item::Potion(crate::game::entity::Potion::Chamomille));
        let test = Test { inventory };
        let json = serde_json::to_string(&test).unwrap();
        let decoded: Test = serde_json::from_str(&json).unwrap();
        assert_eq!(test, decoded);
    }
}
