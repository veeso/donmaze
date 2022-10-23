//! # Items
//!
//! Game items

use super::PlayerState;

mod potions;

pub use potions::Potion;

/// Game items
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(tag = "type")]
pub enum Item {
    /// Makes you know the content of a potion in advance
    AlchemyBook,
    /// Increase max HP and current hp by one
    Armor,
    /// Required to leave the maze
    MazeKey,
    /// A potion with an effect, but you cannot know what it does until you drink it
    Potion(Potion),
    /// Sonar: tells you if there are items or enemies in the adjacent rooms
    Sonar,
    /// Kill any enemy except don maze, which will vanish
    Talisman,
}

impl Item {
    /// Key to use in dictionaries
    pub fn key(&self) -> u32 {
        match self {
            Self::AlchemyBook => 0,
            Self::Armor => 1,
            Self::MazeKey => 2,
            Self::Potion(potion) => potion.key(),
            Self::Sonar => 3,
            Self::Talisman => 4,
        }
    }

    /// Return the item name
    pub fn name(&self, has_alchemy_book: bool) -> &str {
        match self {
            Self::AlchemyBook => "Alchemy book",
            Self::Armor => "Armor",
            Self::MazeKey => "Maze key",
            Self::Potion(_) if !has_alchemy_book => "Potion (???)",
            Self::Potion(potion) => potion.name(),
            Self::Sonar => "Sonar",
            Self::Talisman => "Talisman",
        }
    }

    /// Returns the item description
    pub fn description(&self, has_alchemy_book: bool) -> &str {
        match self {
            Self::AlchemyBook => "Makes you able to know the content of a potion",
            Self::Armor => "Increase max HP by 1",
            Self::MazeKey => "Allows you to leave the maze... once you'll find the exit",
            Self::Potion(_) if !has_alchemy_book => {
                "If only I had an alchemy book or something like that..."
            }
            Self::Potion(potion) => potion.description(),
            Self::Sonar => "Tells you if there are enemies or items in the adjacent rooms",
            Self::Talisman => "Instantly kills an enemy except for don maze, but it seems it will make him disappear",
        }
    }

    /// Returns the effect string
    pub fn effect(&self) -> &str {
        match self {
            Self::AlchemyBook => "",
            Self::Armor => "You worn the armor. Your max HP has been increased by 1",
            Self::MazeKey => "",
            Self::Potion(potion) => potion.effect(),
            Self::Sonar => "The content of the adjacent rooms is revealed",
            Self::Talisman => "You used the ancient power beneath the talisman",
        }
    }

    /// Returns whether the item is consumable
    pub fn consumable(&self) -> bool {
        match self {
            Self::AlchemyBook => false,
            Self::Armor => true,
            Self::MazeKey => false,
            Self::Potion(_) => true,
            Self::Sonar => true,
            Self::Talisman => true,
        }
    }

    /// Returns whether item is usable in current state
    pub fn usable(&self, state: PlayerState) -> bool {
        match (self, state) {
            (
                Self::AlchemyBook,
                PlayerState::Explore | PlayerState::Fight | PlayerState::Asleep,
            ) => false,
            (Self::Armor, PlayerState::Explore | PlayerState::Fight) => true,
            (Self::Armor, PlayerState::Asleep) => false,
            (Self::MazeKey, PlayerState::Explore | PlayerState::Fight | PlayerState::Asleep) => {
                false
            }
            (Self::Potion(_), PlayerState::Explore | PlayerState::Fight) => true,
            (Self::Potion(_), PlayerState::Asleep) => false,
            (Self::Sonar, PlayerState::Explore) => true,
            (Self::Sonar, PlayerState::Asleep | PlayerState::Fight) => false,
            (Self::Talisman, PlayerState::Fight) => true,
            (Self::Talisman, PlayerState::Asleep | PlayerState::Explore) => false,
        }
    }
}

impl From<u32> for Item {
    fn from(key: u32) -> Self {
        match key {
            0 => Self::AlchemyBook,
            1 => Self::Armor,
            2 => Self::MazeKey,
            3 => Self::Sonar,
            4 => Self::Talisman,
            x if x > 255 => Item::Potion(Potion::from(x)),
            _ => Self::Armor, // fallback item
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_item_name() {
        assert_eq!(Item::AlchemyBook.name(false), "Alchemy book");
        assert_eq!(Item::Armor.name(false), "Armor");
        assert_eq!(Item::MazeKey.name(false), "Maze key");
        assert_eq!(Item::Potion(Potion::Chamomille).name(false), "Potion (???)");
        assert_eq!(
            Item::Potion(Potion::Chamomille).name(true),
            Potion::Chamomille.name()
        );
        assert_eq!(Item::Sonar.name(false), "Sonar");
        assert_eq!(Item::Talisman.name(false), "Talisman");
    }

    #[test]
    fn should_get_item_description() {
        assert_eq!(
            Item::AlchemyBook.description(false),
            "Makes you able to know the content of a potion"
        );
        assert_eq!(Item::Armor.description(false), "Increase max HP by 1");
        assert_eq!(
            Item::MazeKey.description(false),
            "Allows you to leave the maze... once you'll find the exit"
        );
        assert_eq!(
            Item::Potion(Potion::Chamomille).description(false),
            "If only I had an alchemy book or something like that..."
        );
        assert_eq!(
            Item::Potion(Potion::Chamomille).description(true),
            Potion::Chamomille.description()
        );
        assert_eq!(
            Item::Sonar.description(false),
            "Tells you if there are enemies or items in the adjacent rooms"
        );
        assert_eq!(
            Item::Talisman.description(false),
            "Instantly kills an enemy except for don maze, but it seems it will make him disappear"
        );
    }

    #[test]
    fn should_get_item_effect() {
        assert_eq!(Item::AlchemyBook.effect(), "");
        assert_eq!(
            Item::Armor.effect(),
            "You worn the armor. Your max HP has been increased by 1"
        );
        assert_eq!(Item::MazeKey.effect(), "");
        assert_eq!(
            Item::Potion(Potion::Chamomille).effect(),
            Potion::Chamomille.effect()
        );
        assert_eq!(
            Item::Sonar.effect(),
            "The content of the adjacent rooms is revealed"
        );
        assert_eq!(
            Item::Talisman.effect(),
            "You used the ancient power beneath the talisman"
        );
    }

    #[test]
    fn should_get_whether_item_is_consumable() {
        assert_eq!(Item::AlchemyBook.consumable(), false);
        assert_eq!(Item::Armor.consumable(), true);
        assert_eq!(Item::MazeKey.consumable(), false);
        assert_eq!(Item::Potion(Potion::Chamomille).consumable(), true);
        assert_eq!(Item::Sonar.consumable(), true);
        assert_eq!(Item::Talisman.consumable(), true);
    }

    #[test]
    fn should_get_whether_item_is_usable() {
        assert_eq!(Item::AlchemyBook.usable(PlayerState::Asleep), false);
        assert_eq!(Item::AlchemyBook.usable(PlayerState::Explore), false);
        assert_eq!(Item::AlchemyBook.usable(PlayerState::Fight), false);
        assert_eq!(Item::Armor.usable(PlayerState::Asleep), false);
        assert_eq!(Item::Armor.usable(PlayerState::Explore), true);
        assert_eq!(Item::Armor.usable(PlayerState::Fight), true);
        assert_eq!(Item::MazeKey.usable(PlayerState::Asleep), false);
        assert_eq!(Item::MazeKey.usable(PlayerState::Explore), false);
        assert_eq!(Item::MazeKey.usable(PlayerState::Fight), false);
        assert_eq!(
            Item::Potion(Potion::Chamomille).usable(PlayerState::Asleep),
            false
        );
        assert_eq!(
            Item::Potion(Potion::Chamomille).usable(PlayerState::Explore),
            true
        );
        assert_eq!(
            Item::Potion(Potion::Chamomille).usable(PlayerState::Fight),
            true
        );
        assert_eq!(Item::Sonar.usable(PlayerState::Asleep), false);
        assert_eq!(Item::Sonar.usable(PlayerState::Explore), true);
        assert_eq!(Item::Sonar.usable(PlayerState::Fight), false);
        assert_eq!(Item::Talisman.usable(PlayerState::Asleep), false);
        assert_eq!(Item::Talisman.usable(PlayerState::Explore), false);
        assert_eq!(Item::Talisman.usable(PlayerState::Fight), true);
    }

    #[test]
    fn should_serialize_items() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct Test {
            objects: Vec<Item>,
        }
        let test = Test {
            objects: vec![Item::AlchemyBook, Item::Potion(Potion::RedPotion)],
        };
        let json = serde_json::to_string(&test).unwrap();
        let decoded: Test = serde_json::from_str(&json).unwrap();
        assert_eq!(test, decoded);
    }

    #[test]
    fn should_convert_items_to_key() {
        assert_eq!(Item::AlchemyBook, Item::from(Item::AlchemyBook.key()));
        assert_eq!(Item::Armor, Item::from(Item::Armor.key()));
        assert_eq!(Item::MazeKey, Item::from(Item::MazeKey.key()));
        assert_eq!(
            Item::Potion(Potion::Chamomille),
            Item::from(Item::Potion(Potion::Chamomille).key())
        );
        assert_eq!(Item::Sonar, Item::from(Item::Sonar.key()));
        assert_eq!(Item::Talisman, Item::from(Item::Talisman.key()));
    }
}
