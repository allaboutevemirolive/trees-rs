// use crate::canva::buffer;

use std::io::StdoutLock;

use crate::canva::buffer::Buffer;

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
    ) -> Self {
        Branch {
            end,
            middle,
            space,
            structural,
        }
    }

    /// Returns the end of a branch, e.g., `"└── "`
    pub fn end(&self) -> &'static str {
        self.end
    }

    /// Returns the middle part of a branch, e.g., `"├── "`
    pub fn middle(&self) -> &'static str {
        self.middle
    }

    /// Returns empty space between branches, e.g., `"    "`
    pub fn space(&self) -> &'static str {
        self.space
    }

    /// Returns the main structural part of the tree, e.g., `"│   "`
    pub fn structural(&self) -> &'static str {
        self.structural
    }

    pub fn paint_branch(&self, value: i32, has_next: bool, buffer: &mut Buffer<StdoutLock>) {
        if has_next {
            if value == 1 {
                buffer.write_branch(self.structural).unwrap();
            } else {
                buffer.write_branch(self.space).unwrap();
            }
        } else {
            if value == 1 {
                buffer.write_branch(self.middle).unwrap();
            } else {
                buffer.write_branch(self.end).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch() {
        // let branch = Branch {
        //     end: "└── ",
        //     middle: "├── ",
        //     space: "    ",
        //     structural: "│   ",
        // };

        let branch = Branch::initialize("└── ", "├── ", "    ", "│   ");

        assert_eq!(branch.end(), "└── ");
        assert_eq!(branch.middle(), "├── ");
        assert_eq!(branch.space(), "    ");
        assert_eq!(branch.structural(), "│   ");
    }
}
