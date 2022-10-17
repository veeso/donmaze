//! # Items
//!
//! Game items

/// Game items
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(tag = "type")]
pub enum Item {
    /// Maze map (kinda OP)
    Map,
    /// Sonar: tells you if there are items, dead-ends or enemies in the next 3 nodes from current node
    Sonar,
    /// Required to leave the maze
    MazeKey,
    /// A potion with an effect, but you cannot know what it does until you drink it
    Potion(Potion),
    /// Increase max HP and current hp by one
    Armor,
    /// Makes you know the content of a potion in advance
    AlchemyBook,
}

/// Potion types
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Potion {
    // -- bonus
    /// Heals 1 HP
    Mead,
    /// Heals 2 HP
    RedPotion,
    /// Heals all HP and increase max HP by 2; kinda rare though
    UnicornElixir,
    // -- malus
    /// Decrease HP by 1
    Vinegar,
    /// Decrease max HP and HP by 1
    DaemonsBlood,
    /// Makes you sleep for 3 turns
    Chamomille,
    /// Decrease HP by 2
    SnakePoison,
    /// it's game over; very rare though
    DeadlyPoison,
}
