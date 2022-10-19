//! # Maze
//!
//! The maze where the game is played

mod generator;
mod room;
use std::collections::HashMap;

use generator::Generator;
use room::Room;

use petgraph::graph::UnGraph;

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Game maze
pub struct Maze {
    /// The node graph
    nodes: UnGraph<u32, u32>,
    /// The data for the rooms
    rooms: HashMap<u32, Room>,
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
    pub fn room_adjacent(&self, room: u32) -> bool {
        self.nodes.contains_edge(self.player.into(), room.into())
    }

    /// Returns adjacent rooms to `room`
    pub fn adjacent_rooms(&self, room: u32) -> Vec<(u32, &Room)> {
        self.nodes
            .edges(room.into())
            .map(|x| (*x.weight(), self.rooms.get(x.weight()).unwrap()))
            .collect()
    }

    /// Get list of rooms
    pub fn rooms(&self) -> Vec<(u32, &Room)> {
        self.rooms.iter().map(|(key, room)| (*key, room)).collect()
    }

    /// Get reference to room by node
    pub fn room(&self, room: u32) -> Option<&Room> {
        self.rooms.get(&room)
    }

    /// Get mutable reference to room by node
    pub fn room_mut(&mut self, room: u32) -> Option<&mut Room> {
        self.rooms.get_mut(&room)
    }

    /// Returns whether current player's room is exit
    pub fn is_exit(&self) -> bool {
        self.rooms.get(&self.player).unwrap().is_exit
    }

    /// Returns whether current player's room has item
    pub fn has_item(&self) -> bool {
        self.rooms.get(&self.player).unwrap().item.is_some()
    }
}
