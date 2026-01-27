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
    /// Computes the diameter of a binary tree via DFS depth calculation.
    ///
    /// # Intuition
    /// The diameter through any node is the sum of left and right subtree
    /// depths. Track the global maximum while computing depths recursively.
    ///
    /// # Approach
    /// 1. DFS returns the depth of each subtree.
    /// 2. At each node, update the diameter as left_depth + right_depth.
    /// 3. Return the maximum diameter seen.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(rc) => {
                    let inner = rc.borrow();
                    let left = dfs(&inner.left, diameter);
                    let right = dfs(&inner.right, diameter);
                    *diameter = (*diameter).max(left + right);
                    1 + left.max(right)
                }
            }
        }
        let mut diameter = 0;
        dfs(&root, &mut diameter);
        diameter
    }
}
