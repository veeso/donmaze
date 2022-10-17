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
    player: i32,
    /// Maze seed
    seed: String,
}

impl Maze {
    /// Generate a brand new maze
    pub fn generate(seed: Option<String>) -> Self {
        Generator::new(seed).generate()
    }
}
