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
    /// Inorder traversal with predecessor tracking for BST validation.
    ///
    /// # Intuition
    /// A valid BST has a strictly increasing inorder traversal. Tracking
    /// the previous value during inorder DFS detects any violation.
    ///
    /// # Approach
    /// Perform inorder DFS, maintaining a mutable `prev` option. After
    /// visiting the left subtree, check that the current value exceeds
    /// `prev`. Update `prev` and recurse right.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once
    /// - Space: O(n) — recursion stack for skewed trees
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
            if let Some(n) = node {
                let n = n.borrow();
                if !dfs(&n.left, prev) {
                    return false;
                }
                if prev.map_or(false, |p| p >= n.val) {
                    return false;
                }
                *prev = Some(n.val);
                dfs(&n.right, prev)
            } else {
                true
            }
        }

        dfs(&root, &mut None)
    }
}
