use rsmgclient::{Connection, Node, Value};
use std::collections::HashMap;
use std::fmt::Display;

mod developer;
mod technology;

pub use developer::Developer;
pub use technology::Technology;

pub enum NodeType {
    Unknown,
    Developer,
    Technology,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Developer => write!(f, "Developer"),
            Self::Technology => write!(f, "Technology"),
        }
    }
}

impl NodeType {
    #[allow(dead_code)]
    pub fn is_type(node: &Node, node_type: NodeType) -> bool {
        node.labels.contains(&node_type.to_string())
    }

    pub fn from(node: &Node) -> Self {
        if node.labels.contains(&Self::Developer.to_string()) {
            Self::Developer
        } else if node.labels.contains(&Self::Technology.to_string()) {
            Self::Technology
        } else {
            Self::Unknown
        }
    }
}

// implementors of trait must implement Display + From<String> + Into<String> traist
pub trait NodeTrait<T>: From<Node> {
    fn new(id: i64, properties: &HashMap<String, Value>) -> T;
    fn create(&self, connection: &mut Connection, node: &T) -> Result<T, Box<dyn std::error::Error>>;
}

pub trait FieldTrait: Display + From<String> + Into<String> {}
