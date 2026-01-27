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
    /// Inserts a value into a maximum binary tree as the last appended element.
    ///
    /// # Intuition
    /// The new value is appended to the original array. If it is larger than
    /// the root, it becomes the new root with the old tree as its left child.
    /// Otherwise, recurse into the right subtree.
    ///
    /// # Approach
    /// If the tree is empty or val > root.val, create a new node with the
    /// existing tree as the left child. Otherwise recursively insert into
    /// the right subtree.
    ///
    /// # Complexity
    /// - Time: O(n) worst case following right spine
    /// - Space: O(n) recursion stack
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(node) if node.borrow().val < val => Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Some(node),
                right: None,
            }))),
            Some(node) => {
                let right = node.borrow_mut().right.take();
                node.borrow_mut().right = Self::insert_into_max_tree(right, val);
                Some(node)
            }
        }
    }
}
