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
    /// Evaluates a full boolean binary tree recursively.
    ///
    /// # Intuition
    /// Leaf nodes hold boolean values (0 or 1). Internal nodes hold operators
    /// (2 = OR, 3 = AND) applied to the results of their children.
    ///
    /// # Approach
    /// 1. Base case: leaf node returns `val == 1`
    /// 2. Recursive case: evaluate both children and apply the operator
    ///
    /// # Complexity
    /// - Time: O(n) — visits every node once
    /// - Space: O(h) — recursion depth equals tree height
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                if node.left.is_none() {
                    return node.val == 1;
                }
                let left = Self::evaluate_tree(node.left.clone());
                let right = Self::evaluate_tree(node.right.clone());
                match node.val {
                    2 => left || right,
                    _ => left && right,
                }
            }
            None => false,
        }
    }
}
