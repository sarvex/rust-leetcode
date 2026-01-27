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
    /// Deletes a node with the given key from a BST.
    ///
    /// # Intuition
    /// Recursively locate the key, then handle three cases: leaf node, single
    /// child, or two children (replace with in-order successor).
    ///
    /// # Approach
    /// 1. Search left or right subtree based on key comparison.
    /// 2. When found with two children, find the leftmost node in the right
    ///    subtree (in-order successor), swap values, and recursively delete.
    /// 3. With zero or one child, return the existing child directly.
    ///
    /// # Complexity
    /// - Time: O(h) where h is tree height
    /// - Space: O(h) recursion depth
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn min_val(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let n = node.as_ref().unwrap().borrow();
            if n.left.is_none() {
                n.val
            } else {
                min_val(&n.left)
            }
        }

        match root {
            None => None,
            Some(rc) => {
                let val = rc.borrow().val;
                match val.cmp(&key) {
                    std::cmp::Ordering::Greater => {
                        let left = rc.borrow_mut().left.take();
                        rc.borrow_mut().left = Self::delete_node(left, key);
                        Some(rc)
                    }
                    std::cmp::Ordering::Less => {
                        let right = rc.borrow_mut().right.take();
                        rc.borrow_mut().right = Self::delete_node(right, key);
                        Some(rc)
                    }
                    std::cmp::Ordering::Equal => {
                        let left = rc.borrow_mut().left.take();
                        let right = rc.borrow_mut().right.take();
                        match (left.is_some(), right.is_some()) {
                            (false, false) => None,
                            (true, false) => left,
                            (false, true) => right,
                            (true, true) => {
                                let successor = min_val(&right);
                                rc.borrow_mut().val = successor;
                                rc.borrow_mut().left = left;
                                rc.borrow_mut().right = Self::delete_node(right, successor);
                                Some(rc)
                            }
                        }
                    }
                }
            }
        }
    }
}
