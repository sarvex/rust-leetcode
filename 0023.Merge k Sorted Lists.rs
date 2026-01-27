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
//     ListNode { next: None, val }
//   }
// }

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val).reverse()
    }
}

impl Solution {
    /// Min-heap (priority queue) merge of k sorted linked lists.
    ///
    /// # Intuition
    /// A min-heap always yields the smallest element among the k list heads.
    /// Extracting the minimum and pushing its successor maintains sorted order.
    ///
    /// # Approach
    /// Collect all non-null heads into a `BinaryHeap` with reversed ordering
    /// (min-heap). Repeatedly pop the smallest node, append its value to the
    /// result list, and push its next node if present.
    ///
    /// # Complexity
    /// - Time: O(N log k) — each of N total nodes inserted/extracted from heap of size k
    /// - Space: O(k) — heap holds at most k nodes simultaneously
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<_> = lists.into_iter().flatten().collect();
        let mut head = None;
        let mut tail = &mut head;

        while let Some(node) = heap.pop() {
            tail = &mut tail.insert(Box::new(ListNode::new(node.val))).next;
            if let Some(next) = node.next {
                heap.push(next);
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: &[i32]) -> Option<Box<ListNode>> {
        vec.iter().rev().fold(None, |next, &val| {
            let mut node = Box::new(ListNode::new(val));
            node.next = next;
            Some(node)
        })
    }

    fn from_list(list: Option<Box<ListNode>>) -> Vec<i32> {
        std::iter::successors(list.as_ref(), |n| n.next.as_ref())
            .map(|n| n.val)
            .collect()
    }

    #[test]
    fn three_sorted_lists() {
        let lists = vec![to_list(&[1, 4, 5]), to_list(&[1, 3, 4]), to_list(&[2, 6])];
        assert_eq!(
            from_list(Solution::merge_k_lists(lists)),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(from_list(Solution::merge_k_lists(vec![])), vec![]);
    }

    #[test]
    fn single_empty_list() {
        assert_eq!(from_list(Solution::merge_k_lists(vec![None])), vec![]);
    }
}
