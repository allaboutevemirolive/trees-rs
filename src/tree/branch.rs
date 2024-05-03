use crate::canva::buffer::Buffer;
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

impl Branch {
    pub fn initialize(
        end: &'static str,
        middle: &'static str,
        space: &'static str,
        structural: &'static str,
    ) -> TResult<Self> {
        Ok(Branch {
            end,
            middle,
            space,
            structural,
        })
    }

    pub fn paint_branch(
        &self,
        value_is_one: bool,
        has_next: bool,
        buffer: &mut Buffer<StdoutLock>,
    ) -> TResult<()> {
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
