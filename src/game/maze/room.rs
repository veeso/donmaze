//! # Room

use crate::game::entity::Enemy;
use crate::game::entity::Item;

/// room stores the information related to a node in the maze graph
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Room {
    pub enemy: Option<Enemy>,
    pub(super) is_exit: bool,
    pub item: Option<Item>,
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct Test {
            room: Room,
        }
        let test = Test {
            room: Room {
                enemy: Some(Enemy::DonMaze),
                is_exit: true,
                item: Some(Item::AlchemyBook),
            },
        };
        let json = serde_json::to_string(&test).unwrap();
        let decoded: Test = serde_json::from_str(&json).unwrap();
        assert_eq!(test, decoded);
    }
}
