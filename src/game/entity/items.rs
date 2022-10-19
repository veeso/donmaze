//! # Items
//!
//! Game items

use super::PlayerState;

/// Game items
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(tag = "type")]
pub enum Item {
    /// Sonar: tells you if there are items or enemies in the adjacent rooms
    Sonar,
    /// Required to leave the maze
    MazeKey,
    /// A potion with an effect, but you cannot know what it does until you drink it
    Potion(Potion),
    /// Increase max HP and current hp by one
    Armor,
    /// Makes you know the content of a potion in advance
    AlchemyBook,
    /// Can OHKO a daemon or make you escape at 100% from DonMaze
    Talisman,
}

impl Item {
    /// Return the item name
    pub fn name(&self, has_alchemy_book: bool) -> &str {
        todo!()
    }

    /// Returns the item description
    pub fn description(&self) -> &str {
        todo!()
    }

    /// Returns the effect string
    pub fn effect(&self) -> &str {
        todo!()
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

/// Potion types
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Potion {
    // -- bonus
    /// Heals 1 HP
    Mead,
    /// Heals 3 HP
    RedPotion,
    /// Heals all HP and increase max HP by 2; kinda rare though
    UnicornElixir,
    // -- malus
    /// Decrease HP by 1
    Vinegar,
    /// Decrease max HP and HP by 1
    DaemonsBlood,
    /// Makes you sleep for 3 turns, but restores 1 HP
    Chamomille,
    /// Decrease HP by 2
    SnakePoison,
    /// it's game over; very rare though
    DeadlyPoison,
}

impl Potion {
    pub fn effect(&self) -> &str {
        todo!()
    }
}
