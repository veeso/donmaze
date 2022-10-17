//! # Maze
//!
//! The maze where the game is played

mod node_data;
use node_data::NodeData;

use petgraph::graph::UnGraph;

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Game maze
pub struct Maze {
    /// The node graph
    graph: UnGraph<i32, NodeData>,
    /// The player position
    player: i32,
}
