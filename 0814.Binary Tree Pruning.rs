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
    /// Prunes subtrees that do not contain a 1 via post-order traversal.
    ///
    /// # Intuition
    /// A subtree should be removed if it contains no 1. Post-order traversal
    /// naturally prunes children before deciding on the parent.
    ///
    /// # Approach
    /// Recursively prune left and right children. If the current node is 0
    /// and both children are pruned (None), remove this node too.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;
        let left = Self::prune_tree(node.borrow_mut().left.take());
        let right = Self::prune_tree(node.borrow_mut().right.take());
        if node.borrow().val == 0 && left.is_none() && right.is_none() {
            return None;
        }
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
        Some(node)
    }
}
