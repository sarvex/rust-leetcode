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
    /// Recursive midpoint selection for height-balanced BST construction.
    ///
    /// # Intuition
    /// Choosing the middle element as root ensures equal-sized subtrees,
    /// producing a height-balanced BST. Recursing on left and right halves
    /// builds the entire tree.
    ///
    /// # Approach
    /// Binary-divide the array. The middle element becomes the root. Left
    /// half builds the left subtree, right half builds the right subtree.
    /// Use index ranges to avoid copying slices.
    ///
    /// # Complexity
    /// - Time: O(n) — each element becomes a node exactly once
    /// - Space: O(log n) — recursion depth for a balanced tree
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(nums: &[i32], left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if left > right || left >= nums.len() {
                return None;
            }
            let mid = left + (right - left) / 2;
            let mut node = TreeNode::new(nums[mid]);
            if mid > 0 {
                node.left = dfs(nums, left, mid - 1);
            }
            node.right = dfs(nums, mid + 1, right);
            Some(Rc::new(RefCell::new(node)))
        }

        if nums.is_empty() {
            return None;
        }
        dfs(&nums, 0, nums.len() - 1)
    }
}
