//! # Node data

/// Node data stores the information related to a node in the maze graph
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeData {
    enemy: Option<()>,
    item: Option<()>,
}
