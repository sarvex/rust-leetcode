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
//     TreeNode { val, left: None, right: None }
//   }
// }

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// Recursive target subtraction for root-to-leaf path sum detection.
    ///
    /// # Intuition
    /// Subtract each node's value from the target as we descend. At a leaf,
    /// check if the remaining target equals the leaf's value.
    ///
    /// # Approach
    /// If the node is None, return false. If it is a leaf, check if its
    /// value equals the remaining target. Otherwise, recurse on both
    /// children with the reduced target.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited at most once
    /// - Space: O(h) — recursion depth equals tree height
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(node) => {
                let node = node.borrow();
                let remaining = target_sum - node.val;
                if node.left.is_none() && node.right.is_none() {
                    return remaining == 0;
                }
                Self::has_path_sum(node.left.clone(), remaining)
                    || Self::has_path_sum(node.right.clone(), remaining)
            }
        }
    }
}
