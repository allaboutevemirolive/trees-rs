use super::branch::PaintBranch;
use crate::render::buffer::Buffer;

use std::io::StdoutLock;

#[derive(Debug, Clone)]
pub struct Node {
    nod: Vec<i32>,
}

impl<'a> IntoIterator for &'a Node {
    type Item = (bool, bool);
    type IntoIter = NodeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator {
            node: self,
            index: 0,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            nod: Vec::with_capacity(5_000_usize),
        }
    }
}

impl Node {
    /// For testing and not intended to be use in production as this will initialize default capacity.
    #[allow(dead_code)]
    fn new(nod: Vec<i32>) -> Self {
        Node { nod }
    }

    pub fn with_capacity(cap: i32) -> anyhow::Result<Self> {
        Ok(Node {
            nod: Vec::with_capacity(cap as usize),
        })
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.nod.pop()
    }

    pub fn push(&mut self, num: i32) {
        self.nod.push(num);
    }

    #[allow(dead_code)]
    fn enumerate_node(&self) -> impl Iterator<Item = (usize, &i32)> {
        self.nod.iter().enumerate()
    }

    /// Get next element or subslice and consume the iterator.
    #[allow(dead_code)]
    fn next_iter<'a>(
        &'a self,
        iter: &mut impl Iterator<Item = (usize, &'a i32)>,
    ) -> Option<(usize, &i32)> {
        iter.next()
    }

    ///  Get reference to the next element or subslice without consuming iterator.
    ///  If next item is out-of-bounds, return None.
    fn next_ref(&self, idx: usize) -> Option<&i32> {
        self.nod.get(idx + 1)
    }

    /// If current entry is not the last entry in entries
    pub fn push_if(&mut self, curr_index: usize, entries_len: usize) {
        if curr_index < entries_len - 1 {
            self.push(1);
        } else {
            self.push(2);
        }
    }

    /// Convert node into branch stick
    pub fn to_branch<T>(&self, branch: &T, buf: &mut Buffer<StdoutLock>) -> anyhow::Result<()>
    where
        T: PaintBranch,
    {
        tracing::info!("Convert node to branch's stick");
        self.into_iter().for_each(|(value_is_one, value_has_next)| {
            branch
                .print_branch_if(value_is_one, value_has_next, buf)
                .expect("Cannot print branch");
        });

        Ok(())
    }
}

pub struct NodeIterator<'a> {
    node: &'a Node,
    index: usize,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = (bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.node.nod.len() {
            let value_is_one = self.node.nod[self.index] == 1;
            let value_has_next = self.node.next_ref(self.index).is_some();
            self.index += 1;
            Some((value_is_one, value_has_next))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test test_node_new -- --nocapture
    #[test]
    fn test_node_new() {
        let node = Node::with_capacity(5).unwrap();
        assert_eq!(node.nod.len(), 0);
        assert_eq!(node.nod.capacity(), 5);
    }

    #[test]
    fn test_push_and_pop() {
        let mut node = Node::with_capacity(5).unwrap();
        node.push(1);
        node.push(2);
        assert_eq!(node.nod.len(), 2);

        let popped = node.pop();
        assert_eq!(popped, Some(2));
        assert_eq!(node.nod.len(), 1);

        let popped = node.pop();
        assert_eq!(popped, Some(1));
        assert_eq!(node.nod.len(), 0);

        let popped = node.pop();
        assert_eq!(popped, None);
    }

    // cargo test test_iterate_node -- --nocapture
    #[test]
    fn test_iterate_node() {
        let mut node = Node::with_capacity(5).unwrap();
        node.push(1);
        node.push(2);
        node.push(3);

        let mut iter = node.enumerate_node();
        assert_eq!(iter.next(), Some((0, &1)));
        assert_eq!(iter.next(), Some((1, &2)));
        assert_eq!(iter.next(), Some((2, &3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterate_node_for_loop() {
        let mut node = Node::with_capacity(5).unwrap();
        node.push(1);
        node.push(2);
        node.push(3);

        let mut expected_iter = node.nod.iter().enumerate();

        for (idx, item) in node.enumerate_node() {
            let (expected_idx, expected_item) = expected_iter.next().unwrap();
            assert_eq!(idx, expected_idx);
            assert_eq!(item, expected_item);
        }
    }
    #[test]
    fn test_next_iter() {
        let mut node = Node::with_capacity(5).unwrap();
        node.push(1);
        node.push(2);
        node.push(3);

        let mut iter = node.enumerate_node();

        assert_eq!(node.next_iter(&mut iter), Some((0, &1)));
        assert_eq!(node.next_iter(&mut iter), Some((1, &2)));
        assert_eq!(node.next_iter(&mut iter), Some((2, &3)));
        assert_eq!(node.next_iter(&mut iter), None);
    }

    #[test]
    fn test_add_entry_marker() {
        let mut node = Node::with_capacity(5).unwrap();
        let num_entries = 5;

        for i in 0..num_entries {
            node.push_if(i, num_entries);
        }

        let mut iter = node.enumerate_node();
        assert_eq!(iter.next(), Some((0, &1)));
        assert_eq!(iter.next(), Some((1, &1)));
        assert_eq!(iter.next(), Some((2, &1)));
        assert_eq!(iter.next(), Some((3, &1)));
        assert_eq!(iter.next(), Some((4, &2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_get_next_item() {
        let mut node = Node::with_capacity(5).unwrap();
        node.push(1);
        node.push(2);
        node.push(3);

        let next_item = node.next_ref(0);
        assert_eq!(next_item, Some(&2));

        let next_item = node.next_ref(1);
        assert_eq!(next_item, Some(&3));

        let next_item = node.next_ref(2);
        assert_eq!(next_item, None);
    }

    #[test]
    fn test_get_next_item_and_check_iterator_movement() {
        let mut node = Node::with_capacity(5).unwrap();
        node.push(1);
        node.push(2);
        node.push(3);

        let mut iter = node.enumerate_node();

        let next_item = node.next_ref(0);
        assert_eq!(next_item, Some(&2));

        assert_eq!(iter.next(), Some((0, &1)));

        let next_item = node.next_ref(1);
        assert_eq!(next_item, Some(&3));

        assert_eq!(iter.next(), Some((1, &2)));

        let next_item = node.next_ref(2);
        assert_eq!(next_item, None);

        assert_eq!(iter.next(), Some((2, &3)));
    }
}
