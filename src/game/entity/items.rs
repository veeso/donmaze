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
    /// Kill any enemy except don maze, which will vanish
    Talisman,
}

impl Item {
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
    pub fn name(&self) -> &str {
        match self {
            Potion::Mead => "Mead",
            Potion::RedPotion => "Red potion",
            Potion::UnicornElixir => "Unicorn elixir",
            Potion::Vinegar => "Vinegar",
            Potion::DaemonsBlood => "Daemon's blood",
            Potion::Chamomille => "Chamomille",
            Potion::SnakePoison => "Snake poison",
            Potion::DeadlyPoison => "Deadly poison",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Potion::Mead => "Restores 1HP",
            Potion::RedPotion => "Restores 3HP",
            Potion::UnicornElixir => "Restores all HP and increase max HP by 2",
            Potion::Vinegar => "Decrease HP by 1",
            Potion::DaemonsBlood => "Decrease HP and max HP by 1",
            Potion::Chamomille => "Put you asleep for 3 turns, but restores 1HP",
            Potion::SnakePoison => "Decrease HP by 2",
            Potion::DeadlyPoison => "Drink it and you will die",
        }
    }

    pub fn effect(&self) -> &str {
        match self {
            Self::Chamomille => "You suddenly feel sleepy, but restored at the same time",
            Self::DaemonsBlood => "Uuugh, that sucks, tastes of iron and rotten flesh, you immediately feel bad",
            Self::DeadlyPoison => {
                "That tastes weirdly..............suddenly you feel a terrible chest pain. You fall on the ground. You start to spit blood from your mouth. And you're dead now"
            }
            Self::Mead => "Slightly alcoholic, but you feel immediately better",
            Self::RedPotion => "Suddenly some legends about a sword and time fill your mind. You immediately feel much better",
            Self::SnakePoison => "The taste of evilness fills your mouth and you feel much worse now",
            Self::UnicornElixir => "That potion tastes like heaven. You feel invincible now",
            Self::Vinegar => "UUugh, it's vinegar. Probably I should have smelled it before drinking it..."
        }
    }
}
