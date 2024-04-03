use self::{branch::Branch, level::Level, node::Node};

pub mod branch;
pub mod level;
pub mod node;

#[derive(Debug, Clone)]
pub struct Tree {
    pub branch: Branch,
    pub level: Level,
    pub nod: Node,
}

impl Tree {
    /// We only need to provide Level struct and node capacity:=.
    pub fn new(level: Level, node_cap: i32) -> Self {
        let branch = Branch::initialize("└── ", "├── ", "    ", "│   ");
        let nod = Node::with_capacity(node_cap);

        Self { nod, branch, level }
    }
}
