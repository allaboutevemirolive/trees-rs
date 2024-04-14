use crate::canva::buffer::Buffer;
use crate::error::simple::UResult;
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
    ) -> UResult<Self> {
        Ok(Branch {
            end,
            middle,
            space,
            structural,
        })
    }

    pub fn paint_branch(
        &self,
        value: i32,
        has_next: bool,
        buffer: &mut Buffer<StdoutLock>,
    ) -> UResult<()> {
        match has_next {
            true => {
                if value == 1 {
                    buffer.write_branch(self.structural)?;
                } else {
                    buffer.write_branch(self.space)?;
                }
            }
            false => {
                if value == 1 {
                    buffer.write_branch(self.middle)?;
                } else {
                    buffer.write_branch(self.end)?;
                }
            }
        }
        Ok(())
    }
}
