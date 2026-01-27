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
    /// Finds the maximum path sum in a binary tree using post-order DFS.
    ///
    /// # Intuition
    /// At each node, the maximum path through it includes the node value plus
    /// the best non-negative contributions from left and right subtrees.
    /// Track the global maximum while returning the best single-branch gain.
    ///
    /// # Approach
    /// 1. Recursively compute the maximum gain from each subtree, clamped to zero.
    /// 2. At each node, update the global max with `val + left_gain + right_gain`.
    /// 3. Return `val + max(left_gain, right_gain)` as the single-branch contribution.
    ///
    /// # Complexity
    /// - Time: O(n) — visit every node once
    /// - Space: O(h) — recursion stack where h is tree height
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MIN;
        Self::dfs(&root, &mut result);
        result
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left = Self::dfs(&node.left, result).max(0);
                let right = Self::dfs(&node.right, result).max(0);
                *result = (*result).max(node.val + left + right);
                node.val + left.max(right)
            }
        }
    }
}
