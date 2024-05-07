use crate::canva::buffer::IntoBranch;
use crate::error::simple::TResult;
use std::io::StdoutLock;

#[derive(Debug, Clone)]
pub struct Branch {
    /// Represents the end of a branch, e.g., `"└── "`
    pub end: &'static str,
    /// Represents the middle part of a branch, e.g., `"├── "`
    pub middle: &'static str,
    /// Represents empty space between branches, e.g., `"    "`
    pub space: &'static str,
    /// Represents the main structural part of the tree, e.g., `"│   "`
    pub structural: &'static str,
}

impl Default for Branch {
    #[rustfmt::skip]
    fn default() -> Self {
        Branch {
            end       : "└── ",
            middle    : "├── ",
            space     : "    ",
            structural: "│   ",
        }
    }
}

pub trait PaintBranch {
    fn paint_branch<'a, T>(
        &self,
        value_is_one: bool,
        has_next: bool,
        buffer: &mut T,
    ) -> TResult<()>
    where
        T: IntoBranch<StdoutLock<'a>>;
}

impl PaintBranch for Branch {
    fn paint_branch<'a, T>(&self, value_is_one: bool, has_next: bool, buffer: &mut T) -> TResult<()>
    where
        T: IntoBranch<StdoutLock<'a>>,
    {
        if has_next {
            if value_is_one {
                buffer.write_branch(self.structural)?;
            } else {
                buffer.write_branch(self.space)?;
            }
        } else {
            if value_is_one {
                buffer.write_branch(self.middle)?;
            } else {
                buffer.write_branch(self.end)?;
            }
        }
        Ok(())
    }
}
