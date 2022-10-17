//! # Room

use crate::game::entity::Enemy;
use crate::game::entity::Item;

/// room stores the information related to a node in the maze graph
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Room {
    pub enemy: Option<Enemy>,
    pub is_exit: bool,
    pub item: Option<Item>,
}
