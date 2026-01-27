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
    /// Finds the longest path where all nodes have the same value.
    ///
    /// # Intuition
    /// DFS returns the longest single-direction path of the target value.
    /// At each node, the path through it equals left + right arms. Track
    /// the global maximum.
    ///
    /// # Approach
    /// 1. DFS with a target value parameter.
    /// 2. If the node matches, extend the path from matching children.
    /// 3. Update the global max with left + right.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target: i32, max_path: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(rc) => {
                    let inner = rc.borrow();
                    let left = dfs(&inner.left, inner.val, max_path);
                    let right = dfs(&inner.right, inner.val, max_path);
                    *max_path = (*max_path).max(left + right);
                    if inner.val == target {
                        left.max(right) + 1
                    } else {
                        0
                    }
                }
            }
        }

        let mut result = 0;
        if let Some(ref rc) = root {
            dfs(&root, rc.borrow().val, &mut result);
        }
        result
    }
}
