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
    /// Counts nodes in a complete binary tree using left-depth comparison.
    ///
    /// # Intuition
    /// In a complete binary tree, if left and right subtree depths are equal,
    /// the left subtree is perfect. Otherwise, the right subtree is perfect
    /// but one level shorter.
    ///
    /// # Approach
    /// 1. Compute left depth of both subtrees.
    /// 2. If equal, left subtree is perfect with `2^depth` nodes; recurse right.
    /// 3. If unequal, right subtree is perfect with `2^depth` nodes; recurse left.
    ///
    /// # Complexity
    /// - Time: O(log^2 n) — log n depth computations of O(log n) each
    /// - Space: O(log n) recursion stack
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left_depth = Self::depth(&node.left);
                let right_depth = Self::depth(&node.right);
                if left_depth == right_depth {
                    Self::count_nodes(node.right.clone()) + (1 << left_depth)
                } else {
                    Self::count_nodes(node.left.clone()) + (1 << right_depth)
                }
            }
        }
    }

    fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => Self::depth(&node.borrow().left) + 1,
        }
    }
}
