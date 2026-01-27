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
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Converts a sorted linked list to a height-balanced BST via array conversion and recursive midpoint splitting.
    ///
    /// # Intuition
    /// A sorted list maps directly to a balanced BST by choosing the middle element as root.
    /// Collecting values into an array enables O(1) midpoint access.
    ///
    /// # Approach
    /// 1. Traverse the linked list and collect all values into a vector.
    /// 2. Recursively select the midpoint of each subarray as root.
    /// 3. Build left and right subtrees from the left and right halves.
    ///
    /// # Complexity
    /// - Time: O(n) for list traversal plus O(n) for tree construction
    /// - Space: O(n) for the values vector plus O(log n) recursion stack
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            vals.push(node.val);
            cur = &node.next;
        }
        Self::build(&vals, 0, vals.len())
    }

    fn build(vals: &[i32], start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            return None;
        }
        let mid = (start + end) >> 1;
        Some(Rc::new(RefCell::new(TreeNode {
            val: vals[mid],
            left: Self::build(vals, start, mid),
            right: Self::build(vals, mid + 1, end),
        })))
    }
}
