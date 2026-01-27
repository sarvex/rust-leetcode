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
    /// Recursive DFS for maximum depth of a binary tree.
    ///
    /// # Intuition
    /// The depth of a tree is 1 plus the maximum depth of its subtrees.
    /// An empty tree has depth 0.
    ///
    /// # Approach
    /// Base case: return 0 for None. Otherwise return 1 plus the max of
    /// left and right subtree depths.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once
    /// - Space: O(h) — recursion depth equals tree height
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    1 + dfs(&n.left).max(dfs(&n.right))
                }
            }
        }

        dfs(&root)
    }
}
