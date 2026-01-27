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
    /// Trims a BST to only contain values in [low, high].
    ///
    /// # Intuition
    /// If a node's value is below low, its entire left subtree is also out of
    /// range — recurse on the right. Similarly for above high.
    ///
    /// # Approach
    /// 1. If val < low, return trimmed right subtree.
    /// 2. If val > high, return trimmed left subtree.
    /// 3. Otherwise, recursively trim both children.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(rc) => {
                let val = rc.borrow().val;
                if val < low {
                    let right = rc.borrow_mut().right.take();
                    Self::trim_bst(right, low, high)
                } else if val > high {
                    let left = rc.borrow_mut().left.take();
                    Self::trim_bst(left, low, high)
                } else {
                    let left = rc.borrow_mut().left.take();
                    let right = rc.borrow_mut().right.take();
                    rc.borrow_mut().left = Self::trim_bst(left, low, high);
                    rc.borrow_mut().right = Self::trim_bst(right, low, high);
                    Some(rc)
                }
            }
        }
    }
}
