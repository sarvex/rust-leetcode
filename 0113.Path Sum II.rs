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
    /// Finds all root-to-leaf paths that sum to a target using backtracking DFS.
    ///
    /// # Intuition
    /// Explore every root-to-leaf path, maintaining a running path and remaining sum.
    /// At each leaf, check if the remaining sum equals the node's value.
    ///
    /// # Approach
    /// 1. Use DFS with a mutable path vector for backtracking.
    /// 2. Subtract each node's value from the remaining target.
    /// 3. At leaf nodes, if remaining target is zero, record the current path.
    /// 4. Pop the last element after exploring both subtrees.
    ///
    /// # Complexity
    /// - Time: O(n^2) — visit every node, path cloning at leaves costs O(n)
    /// - Space: O(n) — recursion stack and path storage
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        Self::dfs(&root, target_sum, &mut path, &mut result);
        result
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        remaining: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            let remaining = remaining - node.val;
            path.push(node.val);
            if node.left.is_none() && node.right.is_none() {
                if remaining == 0 {
                    result.push(path.clone());
                }
            } else {
                Self::dfs(&node.left, remaining, path, result);
                Self::dfs(&node.right, remaining, path, result);
            }
            path.pop();
        }
    }
}
