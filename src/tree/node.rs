use std::io::StdoutLock;

use crate::{canva::buffer::Buffer, error::simple::TResult};

use super::branch::Branch;

#[derive(Debug, Clone)]
pub struct Node {
    pub nod: Vec<i32>,
}

impl Node {
    /// For testing and not intended to be use in production as
    /// this will initialize default capacity.
    #[allow(dead_code)]
    fn new(nod: Vec<i32>) -> Self {
        Node { nod }
    }

    pub fn with_capacity(cap: i32) -> TResult<Self> {
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

    /// If there is remaining folder needs to be traverse
    pub fn mark_entry(&mut self, curr_index: usize, num_entries: usize) {
        if curr_index < num_entries - 1 {
            self.push(1);
        } else {
            self.push(2);
        }
    }

    pub fn to_branches(&mut self, br: &Branch, buf: &mut Buffer<StdoutLock>) -> TResult<()> {
        self.into_iter().for_each(|(is_one, has_next)| {
            br.paint_branch(is_one, has_next, buf)
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
            let curr_value_is_one = self.node.nod[self.index] == 1;
            let has_next = self.node.next_ref(self.index).is_some();
            self.index += 1;
            Some((curr_value_is_one, has_next))
        } else {
            None
        }
    }
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

        // Add markers for each entry except the last one
        for i in 0..num_entries {
            node.mark_entry(i, num_entries);
        }

        // Check the contents of the node
        let mut iter = node.enumerate_node();
        assert_eq!(iter.next(), Some((0, &1))); // First entry marker
        assert_eq!(iter.next(), Some((1, &1))); // Second entry marker
        assert_eq!(iter.next(), Some((2, &1))); // Third entry marker
        assert_eq!(iter.next(), Some((3, &1))); // Fourth entry marker
        assert_eq!(iter.next(), Some((4, &2))); // Last entry marker
        assert_eq!(iter.next(), None); // No more entries
    }

    #[test]
    fn test_get_next_item() {
        let mut node = Node::with_capacity(5).unwrap();
        node.push(1);
        node.push(2);
        node.push(3);

        // let iter = node.enumerate_node();

        // Test for the next item after the first element
        let next_item = node.next_ref(0);
        assert_eq!(next_item, Some(&2));

        // Test for the next item after the second element
        // let _ = iter.next(); // Consume the iterator to point to the second element
        let next_item = node.next_ref(1);
        assert_eq!(next_item, Some(&3));

        // Test for the next item after the last element
        // let _ = iter.next(); // Consume the iterator to point to the third element
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

        // Test for the next item after the first element
        let next_item = node.next_ref(0);
        assert_eq!(next_item, Some(&2));
        // Assert that the iterator remains at the first element
        assert_eq!(iter.next(), Some((0, &1)));

        // Test for the next item after the second element
        let next_item = node.next_ref(1);
        assert_eq!(next_item, Some(&3));
        // Assert that the iterator remains at the second element
        assert_eq!(iter.next(), Some((1, &2)));

        // Test for the next item after the last element
        let next_item = node.next_ref(2);
        assert_eq!(next_item, None);
        // Assert that the iterator remains at the third element
        assert_eq!(iter.next(), Some((2, &3)));
    }

    // cargo test test_node_iterator -- --nocapture
    // #[test]
    // fn test_node_iterator() {
    //     let node = Node::new(vec![1, 2, 3]);
    //     let mut iterator = node.into_iter();
    //     // for (value, has_next) in iterator {
    //     // }
    //     assert_eq!(iterator.next(), Some((1, true)));
    //     assert_eq!(iterator.next(), Some((2, true)));
    //     assert_eq!(iterator.next(), Some((3, false)));
    //     assert_eq!(iterator.next(), None);
    // }

    // #[test]
    // fn test_node_iterator_for_loop() {
    //     let node = Node::new(vec![1, 2, 3]);
    //     let mut iterator = node.into_iter();

    //     let expected_values = vec![(1, true), (2, true), (3, false)];
    //     for expected_value in &expected_values {
    //         assert_eq!(iterator.next(), Some(*expected_value));
    //     }
    //     assert_eq!(iterator.next(), None);
    // }

    // #[test]
    // fn test_node_iterator_while_loop() {
    //     let node = Node::new(vec![1, 2, 3]);
    //     let mut iterator = node.into_iter();

    //     let expected_values = vec![(1, true), (2, true), (3, false)];
    //     let mut index = 0;
    //     while let Some(expected_value) = expected_values.get(index) {
    //         assert_eq!(iterator.next(), Some(*expected_value));
    //         index += 1;
    //     }
    //     assert_eq!(iterator.next(), None);
    // }

    // cargo test test_iteration_and_next -- --nocapture
    // #[test]
    // fn test_iteration_and_next() {
    //     let node = Node {
    //         nod: vec![1, 2, 3, 4, 5],
    //     };
    //     let mut iter = node.iter();

    //     let mut expected_index = 0;
    //     while let Some((index, item)) = iter.next() {
    //         assert_eq!(index, expected_index);
    //         assert_eq!(*item, node.nod[index]);
    //         expected_index += 1;

    //         if let Some((next_index, next_item)) = iter.get_next() {
    //             assert_eq!(next_index, expected_index);
    //             assert_eq!(*next_item, node.nod[next_index]);
    //         } else {
    //             assert_eq!(expected_index, node.nod.len());
    //         }
    //     }
    // }
}
