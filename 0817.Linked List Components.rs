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
use std::collections::HashSet;

impl Solution {
    /// Counts connected components in a linked list subset.
    ///
    /// # Intuition
    /// A connected component starts whenever a node is in the subset and its
    /// predecessor is not. Traverse the list tracking membership transitions.
    ///
    /// # Approach
    /// Store the subset in a `HashSet`. Walk the list, counting transitions
    /// from outside the set to inside.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(m) where m is the subset size
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut count = 0;
        let mut in_component = false;
        let mut cur = &head;
        while let Some(node) = cur {
            if set.contains(&node.val) {
                if !in_component {
                    in_component = true;
                    count += 1;
                }
            } else {
                in_component = false;
            }
            cur = &node.next;
        }
        count
    }
}
