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
    /// Finds minimum depth of a binary tree using recursive DFS with leaf-awareness.
    ///
    /// # Intuition
    /// Minimum depth is the shortest root-to-leaf path. A node with only one child
    /// is not a leaf, so we must continue down the non-null subtree.
    ///
    /// # Approach
    /// 1. Return 0 for an empty tree.
    /// 2. If one child is missing, recurse into the other child.
    /// 3. If both children exist, take the minimum of their depths.
    ///
    /// # Complexity
    /// - Time: O(n) — visit every node once
    /// - Space: O(h) — recursion stack where h is tree height
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => 1,
                    (None, right) => 1 + Self::dfs(right),
                    (left, None) => 1 + Self::dfs(left),
                    (left, right) => 1 + Self::dfs(left).min(Self::dfs(right)),
                }
            }
        }
    }
}
