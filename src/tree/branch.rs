use crate::error::simple::TResult;
use crate::render::buffer::IntoBranch;

use std::io::StdoutLock;

#[derive(Debug, Clone)]
pub struct Branch {
    /// Represents the end of a branch, e.g., "└── "
    end: &'static str,
    /// Represents the middle part of a branch, e.g., "├── "
    middle: &'static str,
    /// Represents empty space between branches, e.g., "    "
    space: &'static str,
    /// Represents the main structural part of the tree, e.g., "│   "
    structural: &'static str,
}

impl Branch {
    pub fn no_branch(&mut self) {
        self.end = "";
        self.middle = "";
        self.space = "";
        self.structural = "";
    }
}

impl Default for Branch {
    #[rustfmt::skip]
    fn default() -> Self {
        Branch {
            end:        "└── ",
            middle:     "├── ",
            space:      "    ",
            structural: "│   ",
        }
    }
}

pub trait PaintBranch {
    fn print_branch_if<'a, T>(
        &self,
        value_is_one: bool,
        value_has_next: bool,
        buffer: &mut T,
    ) -> TResult<()>
    where
        T: IntoBranch<StdoutLock<'a>>;
}

impl PaintBranch for Branch {
    #[allow(clippy::collapsible_else_if)]
    fn print_branch_if<'a, T>(
        &self,
        value_is_one: bool,
        value_has_next: bool,
        buffer: &mut T,
    ) -> TResult<()>
    where
        T: IntoBranch<StdoutLock<'a>>,
    {
        if value_has_next {
            if value_is_one {
                buffer.print_branch(self.structural)?;
            } else {
                buffer.print_branch(self.space)?;
            }
        } else {
            if value_is_one {
                buffer.print_branch(self.middle)?;
            } else {
                buffer.print_branch(self.end)?;
            }
        }
        Ok(())
    }
}
