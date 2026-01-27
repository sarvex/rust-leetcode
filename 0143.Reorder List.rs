// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::collections::VecDeque;

impl Solution {
    /// Reorders a linked list by interleaving from both ends using a deque.
    ///
    /// # Intuition
    /// Collect all nodes (except head) into a deque, then alternately pop from
    /// the back and front to interleave the list.
    ///
    /// # Approach
    /// 1. Detach all nodes after the head into a VecDeque.
    /// 2. Alternately pop from back and front, relinking nodes.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the deque
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut tail = &mut head.as_mut().unwrap().next;
        let mut current = tail.take();
        let mut deque = VecDeque::new();
        while let Some(mut node) = current {
            current = node.next.take();
            deque.push_back(Some(node));
        }
        let mut from_back = true;
        while !deque.is_empty() {
            *tail = if from_back {
                deque.pop_back().unwrap()
            } else {
                deque.pop_front().unwrap()
            };
            tail = &mut tail.as_mut().unwrap().next;
            from_back = !from_back;
        }
    }
}
