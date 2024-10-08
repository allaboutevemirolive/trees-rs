use super::branch::PaintBranch;
use crate::render::buffer::Buffer;
use anyhow::Result;
use std::io::StdoutLock;

const DEFAULT_CAPACITY: usize = 5_000;

#[derive(Debug, Clone)]
pub struct Node {
    values: Vec<i32>,
}

impl Default for Node {
    fn default() -> Self {
        Self::with_capacity_unchecked(DEFAULT_CAPACITY)
    }
}

impl Node {
    /// Creates a new Node with the given values.
    ///
    /// # Note
    /// For testing purposes only. Not intended for production use.
    #[cfg(test)]
    pub fn new(values: Vec<i32>) -> Self {
        Self { values }
    }

    /// Creates a new Node with the specified capacity.
    pub fn with_capacity(cap: i32) -> Result<Self> {
        if cap < 0 {
            anyhow::bail!("Capacity must be non-negative");
        }
        Ok(Self::with_capacity_unchecked(cap as usize))
    }

    /// Internal method to create a Node with unchecked capacity.
    fn with_capacity_unchecked(cap: usize) -> Self {
        Self {
            values: Vec::with_capacity(cap),
        }
    }

    #[inline(always)]
    /// Removes and returns the last element.
    pub fn pop(&mut self) -> Option<i32> {
        self.values.pop()
    }

    #[inline(always)]
    /// Appends an element to the end.
    pub fn push(&mut self, num: i32) {
        self.values.push(num);
    }

    /// Returns an iterator over the values with their indices.
    pub fn enumerate(&self) -> impl Iterator<Item = (usize, &i32)> {
        self.values.iter().enumerate()
    }

    /// Gets the next element from an iterator.
    pub fn next_from_iterator<'a, I>(&'a self, iter: &mut I) -> Option<(usize, &'a i32)>
    where
        I: Iterator<Item = (usize, &'a i32)>,
    {
        iter.next()
    }

    /// Gets a reference to the next element without consuming the iterator.
    fn peek_next(&self, current_index: usize) -> Option<&i32> {
        self.values.get(current_index + 1)
    }

    #[inline(always)]
    /// Pushes a value based on the current index and total entries.
    pub fn push_conditional(&mut self, current_index: usize, total_entries: usize) {
        let value = if current_index < total_entries - 1 {
            1
        } else {
            2
        };
        self.push(value);
    }

    #[inline(always)]
    /// Converts the node into a branch representation.
    pub fn paint_as_branch<T>(&self, branch: &T, buffer: &mut Buffer<StdoutLock>) -> Result<()>
    where
        T: PaintBranch,
    {
        tracing::info!("Converting node to branch representation");

        for (is_one, has_next) in self.into_iter() {
            branch
                .print_branch_if(is_one, has_next, buffer)
                .map_err(|e| anyhow::anyhow!("Failed to print branch: {}", e))?;
        }

        Ok(())
    }

    /// Returns the current length of the node.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if the node contains no elements.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<'a> IntoIterator for &'a Node {
    type Item = (bool, bool);
    type IntoIter = NodeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator::new(self)
    }
}

pub struct NodeIterator<'a> {
    node: &'a Node,
    index: usize,
}

impl<'a> NodeIterator<'a> {
    fn new(node: &'a Node) -> Self {
        Self { node, index: 0 }
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = (bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.node.len() {
            let current_value = &self.node.values[self.index];
            let is_one = *current_value == 1;
            let has_next = self.node.peek_next(self.index).is_some();

            self.index += 1;
            Some((is_one, has_next))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::default();
        assert_eq!(node.len(), 0);
        assert!(node.is_empty());
    }

    #[test]
    fn test_push_and_pop() {
        let mut node = Node::default();
        node.push(1);
        node.push(2);

        assert_eq!(node.pop(), Some(2));
        assert_eq!(node.pop(), Some(1));
        assert_eq!(node.pop(), None);
    }

    #[test]
    fn test_invalid_capacity() {
        assert!(Node::with_capacity(-1).is_err());
    }

    #[test]
    fn test_iterator() {
        let node = Node::new(vec![1, 2, 1]);
        let items: Vec<_> = (&node).into_iter().collect();
        assert_eq!(items, vec![(true, true), (false, true), (true, false)]);
    }
}
