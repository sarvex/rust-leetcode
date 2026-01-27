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
    /// Bottom-up height check with early termination for balanced tree validation.
    ///
    /// # Intuition
    /// A balanced tree has subtree heights differing by at most 1 at every node.
    /// Computing height bottom-up and returning -1 on imbalance propagates
    /// failure without redundant traversals.
    ///
    /// # Approach
    /// DFS returns the height of each subtree, or -1 if any subtree is
    /// unbalanced. At each node, if either child returns -1 or the height
    /// difference exceeds 1, return -1. Otherwise return `1 + max(left, right)`.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once
    /// - Space: O(h) — recursion depth equals tree height
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let left = height(&n.left);
                    let right = height(&n.right);
                    if left == -1 || right == -1 || (left - right).abs() > 1 {
                        -1
                    } else {
                        1 + left.max(right)
                    }
                }
            }
        }

        height(&root) != -1
    }
}
