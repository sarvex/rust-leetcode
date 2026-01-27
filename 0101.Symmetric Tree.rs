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
    /// Mirror-comparison DFS for symmetric tree validation.
    ///
    /// # Intuition
    /// A tree is symmetric if its left subtree is a mirror of its right
    /// subtree. Two subtrees are mirrors when their roots match and each
    /// node's left child mirrors the other's right child.
    ///
    /// # Approach
    /// Compare the root's left and right children. Recursively verify
    /// that `left.left` mirrors `right.right` and `left.right` mirrors
    /// `right.left`.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once
    /// - Space: O(h) — recursion depth equals tree height
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirror(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    let (a, b) = (a.borrow(), b.borrow());
                    a.val == b.val && is_mirror(&a.left, &b.right) && is_mirror(&a.right, &b.left)
                }
                _ => false,
            }
        }

        match root {
            Some(r) => {
                let r = r.borrow();
                is_mirror(&r.left, &r.right)
            }
            None => true,
        }
    }
}
