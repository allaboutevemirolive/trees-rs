use crate::render::buffer::IntoBranch;
use std::io::StdoutLock;

#[derive(Debug, Clone)]
pub struct Branch {
    end: &'static str,
    middle: &'static str,
    space: &'static str,
    structural: &'static str,
}

impl Branch {
    /// Resets the branch to have no structural markers.
    pub fn no_branch(&mut self) {
        self.reset_branches("", "", "", "");
    }

    /// Resets branch properties with the given values.
    fn reset_branches(
        &mut self,
        end: &'static str,
        middle: &'static str,
        space: &'static str,
        structural: &'static str,
    ) {
        self.end = end;
        self.middle = middle;
        self.space = space;
        self.structural = structural;
    }
}

impl Default for Branch {
    fn default() -> Self {
        Branch {
            end: "└── ",
            middle: "├── ",
            space: "    ",
            structural: "│   ",
        }
    }
}

/// A trait for painting branches onto a buffer.
pub trait PaintBranch {
    fn print_branch_if<'a, T>(
        &self,
        value_is_one: bool,
        value_has_next: bool,
        buffer: &mut T,
    ) -> anyhow::Result<()>
    where
        T: IntoBranch<StdoutLock<'a>>;
}

impl PaintBranch for Branch {
    fn print_branch_if<'a, T>(
        &self,
        value_is_one: bool,
        value_has_next: bool,
        buffer: &mut T,
    ) -> anyhow::Result<()>
    where
        T: IntoBranch<StdoutLock<'a>>,
    {
        let branch_part = self.determine_branch_part(value_is_one, value_has_next);
        buffer.print_branch(branch_part)?;
        Ok(())
    }
}

impl Branch {
    /// Determines which branch part to print based on the conditions.
    fn determine_branch_part(&self, value_is_one: bool, value_has_next: bool) -> &'static str {
        if value_has_next {
            if value_is_one {
                self.structural
            } else {
                self.space
            }
        } else {
            if value_is_one {
                self.middle
            } else {
                self.end
            }
        }
    }
}
