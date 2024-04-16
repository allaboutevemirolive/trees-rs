pub mod branch;
use self::branch::Branch;

pub mod level;
use self::level::Level;

pub mod node;
use self::node::Node;

use crate::error::simple::UResult;
use crate::error::simple::USimpleError;

#[derive(Debug, Clone)]
pub struct Tree {
    pub branch: Branch,
    pub level: Level,
    pub nod: Node,
}

impl Tree {
    pub fn new(level: Level, node_cap: i32) -> UResult<Self> {
        let branch = Branch::initialize("└── ", "├── ", "    ", "│   ")
            .map_err(|err| USimpleError::new(1, format!("Failed to initialize branch: {}", err)))?;

        let nod = Node::with_capacity(node_cap)
            .map_err(|err| USimpleError::new(1, format!("Failed to initialize node: {}", err)))?;

        Ok(Self { nod, branch, level })
    }
}

impl PartialEq for Branch {
    fn eq(&self, other: &Self) -> bool {
        self.end == other.end
            && self.middle == other.middle
            && self.space == other.space
            && self.structural == other.structural
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test test_branch_initialize -- --nocapture
    #[test]
    fn test_branch_initialize() {
        let expected_branch = Branch {
            end: "└── ",
            middle: "├── ",
            space: "    ",
            structural: "│   ",
        };

        let branch = Branch::initialize("└── ", "├── ", "    ", "│   ").unwrap();

        assert_eq!(branch, expected_branch);
    }
}
