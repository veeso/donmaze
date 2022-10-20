//! # Maze
//!
//! The maze where the game is played

mod generator;
mod room;

use crate::utils::graphq;
use generator::Generator;

use super::entity::{Enemy, Item};
use room::Room;

use petgraph::{graph::UnGraph, stable_graph::DefaultIx, visit::EdgeRef};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Game maze
pub struct Maze {
    /// The node graph
    nodes: UnGraph<u32, u32>,
    /// The data for the rooms
    rooms: HashMap<DefaultIx, Room>,
    /// The player position
    pub player: u32,
    /// Maze seed
    seed: String,
}

impl Maze {
    /// Generate a brand new maze
    pub fn generate(seed: Option<String>) -> Self {
        Generator::new(seed).generate()
    }

    /// Get maze seed
    pub fn seed(&self) -> &str {
        &self.seed
    }

    /// Check whether provided room is adjacent to room player
    pub fn room_adjacent(&self, room: DefaultIx) -> bool {
        self.nodes.contains_edge(self.player.into(), room.into())
    }

    /// Returns adjacent rooms to `room`
    pub fn adjacent_rooms(&self, room: DefaultIx) -> Vec<(DefaultIx, &Room)> {
        self.nodes
            .edges(room.into())
            .map(|x| {
                let room = if x.source() == room.into() {
                    x.target()
                } else {
                    x.source()
                }
                .index() as u32;
                (room, self.rooms.get(&room).unwrap())
            })
            .collect()
    }

    /// Get list of rooms
    pub fn rooms(&self) -> Vec<(DefaultIx, &Room)> {
        self.rooms.iter().map(|(key, room)| (*key, room)).collect()
    }

    /// Get reference to room by node
    pub fn room(&self, room: DefaultIx) -> Option<&Room> {
        self.rooms.get(&room)
    }

    /// Get mutable reference to room by node
    pub fn room_mut(&mut self, room: DefaultIx) -> Option<&mut Room> {
        self.rooms.get_mut(&room)
    }

    /// Returns whether current player's room is exit
    pub fn is_exit(&self) -> bool {
        self.rooms.get(&self.player).unwrap().is_exit
    }

    /// Returns whether current player's room has item
    pub fn has_item(&self) -> bool {
        self.room(self.player).unwrap().item.is_some()
    }

    /// Take the item from the player's room
    pub fn take_item(&mut self) -> Option<Item> {
        self.room_mut(self.player).as_mut().unwrap().item.take()
    }

    /// Take enemy from the player's room
    pub fn take_enemy(&mut self) -> Option<Enemy> {
        self.room_mut(self.player).as_mut().unwrap().enemy.take()
    }

    /// Get reference to fighting enemy (if any)
    pub fn fighting_enemy(&self) -> Option<&Enemy> {
        self.room(self.player).map(|x| x.enemy.as_ref()).flatten()
    }

    /// Get mutable reference to fighting enemy
    pub fn fighting_enemy_mut(&mut self) -> Option<&mut Enemy> {
        self.room_mut(self.player)
            .map(|x| x.enemy.as_mut())
            .flatten()
    }
}

impl PartialEq for Maze {
    fn eq(&self, other: &Self) -> bool {
        if self.player != other.player {
            return false;
        }
        if self.seed != other.seed {
            return false;
        }
        if self.rooms.len() != other.rooms.len() {
            return false;
        }
        if self.rooms != other.rooms {
            return false;
        }
        graphq::graph_eq(&self.nodes, &other.nodes)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::game::entity::{Enemy, Item};

    use pretty_assertions::assert_eq;

    #[test]
    fn should_tell_whether_room_is_adjacent() {
        let maze = fake_maze();
        assert_eq!(maze.room_adjacent(1), true);
        assert_eq!(maze.room_adjacent(2), true);
        assert_eq!(maze.room_adjacent(3), false);
    }

    #[test]
    fn should_get_adjacent_rooms() {
        let maze = fake_maze();
        let mut adjacent_rooms = maze.adjacent_rooms(4);
        adjacent_rooms.sort_by_key(|(node, _)| *node);
        assert_eq!(adjacent_rooms.len(), 4);
        assert_eq!(
            adjacent_rooms[0],
            (
                2,
                &Room {
                    enemy: Some(Enemy::Daemon(crate::game::entity::Daemon::new(4))),
                    is_exit: false,
                    item: None,
                }
            )
        );
        assert_eq!(adjacent_rooms[1], (5, &Room::default()));
        assert_eq!(adjacent_rooms[2], (6, &Room::default()));
        assert_eq!(
            adjacent_rooms[3],
            (
                7,
                &Room {
                    enemy: None,
                    is_exit: true,
                    item: None,
                }
            )
        );
    }

    #[test]
    fn should_get_rooms() {
        let maze = fake_maze();
        assert_eq!(maze.rooms().len(), 9);
    }

    #[test]
    fn should_mut_rooms() {
        let mut maze = fake_maze();
        let room = maze.room_mut(6).unwrap();
        room.item = Some(Item::Sonar);
        let room = maze.room(6).unwrap();
        assert_eq!(room.item.unwrap(), Item::Sonar);
        assert!(maze.room(60).is_none());
    }

    #[test]
    fn should_tell_whether_is_exit() {
        let mut maze = fake_maze();
        assert_eq!(maze.is_exit(), false);
        maze.player = 7;
        assert_eq!(maze.is_exit(), true);
    }

    #[test]
    fn should_tell_whether_room_has_item() {
        let mut maze = fake_maze();
        assert_eq!(maze.has_item(), false);
        maze.player = 1;
        assert_eq!(maze.has_item(), true);
    }

    #[test]
    fn should_take_item() {
        let mut maze = fake_maze();
        assert_eq!(maze.take_item(), None);
        maze.player = 1;
        assert_eq!(maze.take_item(), Some(Item::Armor));
        assert_eq!(maze.take_item(), None);
    }

    #[test]
    fn should_take_enemy() {
        let mut maze = fake_maze();
        assert_eq!(maze.take_enemy(), None);
        maze.player = 2;
        assert_eq!(
            maze.take_enemy(),
            Some(Enemy::Daemon(crate::game::entity::Daemon::new(4)))
        );
        assert_eq!(maze.take_enemy(), None);
    }

    #[test]
    fn should_get_fighting_enemy() {
        let mut maze = fake_maze();
        assert_eq!(maze.fighting_enemy(), None);
        maze.player = 2;
        assert_eq!(
            maze.fighting_enemy(),
            Some(&Enemy::Daemon(crate::game::entity::Daemon::new(4)))
        );
        let enemy = maze.fighting_enemy_mut().unwrap();
        enemy.damage(1);
        assert_eq!(
            maze.fighting_enemy(),
            Some(&Enemy::Daemon(crate::game::entity::Daemon::new(3)))
        );
    }

    fn fake_maze() -> Maze {
        let nodes = fake_maze_graph();
        // add rooms
        let mut rooms = HashMap::new();
        rooms.insert(0, Room::default());
        rooms.insert(
            1,
            Room {
                enemy: None,
                is_exit: false,
                item: Some(Item::Armor),
            },
        );
        rooms.insert(
            2,
            Room {
                enemy: Some(Enemy::Daemon(crate::game::entity::Daemon::new(4))),
                is_exit: false,
                item: None,
            },
        );
        rooms.insert(3, Room::default());
        rooms.insert(4, Room::default());
        rooms.insert(5, Room::default());
        rooms.insert(6, Room::default());
        rooms.insert(
            7,
            Room {
                enemy: None,
                is_exit: true,
                item: None,
            },
        );
        rooms.insert(8, Room::default());

        Maze {
            nodes,
            rooms,
            player: 0,
            seed: String::from("test"),
        }
    }

    fn fake_maze_graph() -> UnGraph<u32, u32> {
        /*
         * 0
         *  - 1
         *  \
         *   -> 3
         *
         * - 2
         *  \
         *   -> 4
         *    \ -> 5
         *    \ -> 6
         *    \ -> 7
         *         \
         *          -> 8
         *          -> exit
         */
        let mut nodes: UnGraph<u32, u32> = UnGraph::default();
        nodes.add_node(0);
        nodes.add_node(1);
        nodes.add_node(2);
        nodes.add_node(3);
        nodes.add_node(4);
        nodes.add_node(5);
        nodes.add_node(6);
        nodes.add_node(7);
        nodes.add_node(8);
        nodes.add_edge(0.into(), 1.into(), 0);
        nodes.add_edge(0.into(), 2.into(), 0);
        nodes.add_edge(1.into(), 3.into(), 0);
        nodes.add_edge(2.into(), 4.into(), 0);
        nodes.add_edge(4.into(), 5.into(), 0);
        nodes.add_edge(4.into(), 6.into(), 0);
        nodes.add_edge(4.into(), 7.into(), 0);
        nodes.add_edge(7.into(), 8.into(), 0);
        nodes
    }
}
