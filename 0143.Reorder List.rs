#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
        if head.is_none() {
            return;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a linked list from a vector
    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for val in vec.into_iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    /// Helper function to convert a linked list to a vector
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_reorder_list_example1() {
        // Input: [1,2,3,4]
        // Expected: [1,4,2,3]
        let mut head = vec_to_list(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut head);
        assert_eq!(list_to_vec(head), vec![1, 4, 2, 3]);
    }

    #[test]
    fn test_reorder_list_example2() {
        // Input: [1,2,3,4,5]
        // Expected: [1,5,2,4,3]
        let mut head = vec_to_list(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut head);
        assert_eq!(list_to_vec(head), vec![1, 5, 2, 4, 3]);
    }

    #[test]
    fn test_reorder_list_single_node() {
        // Input: [1]
        // Expected: [1]
        let mut head = vec_to_list(vec![1]);
        Solution::reorder_list(&mut head);
        assert_eq!(list_to_vec(head), vec![1]);
    }

    #[test]
    fn test_reorder_list_two_nodes() {
        // Input: [1,2]
        // Expected: [1,2]
        let mut head = vec_to_list(vec![1, 2]);
        Solution::reorder_list(&mut head);
        assert_eq!(list_to_vec(head), vec![1, 2]);
    }

    #[test]
    fn test_reorder_list_empty() {
        // Input: []
        // Expected: []
        let mut head = None;
        Solution::reorder_list(&mut head);
        assert_eq!(head, None);
    }
}
