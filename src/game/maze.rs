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

    /// Check whether provided room is adjacent to room player
    pub fn room_adjacent(&self, room: u32) -> bool {
        self.nodes.contains_edge(self.player.into(), room.into())
    }

    /// Returns adjacent rooms to `room`
    pub fn adjacent_rooms(&self, room: u32) -> Vec<u32> {
        self.nodes.edges(room.into()).map(|x| *x.weight()).collect()
    }
}
