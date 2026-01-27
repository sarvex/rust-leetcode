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
    /// Finds the kth smallest element in a BST using early-terminating inorder DFS.
    ///
    /// # Intuition
    /// Inorder traversal of a BST yields elements in sorted order. Stop
    /// as soon as k elements have been collected.
    ///
    /// # Approach
    /// 1. Perform inorder DFS, collecting values into a vector.
    /// 2. Early-terminate once k elements are collected.
    /// 3. Return the kth element.
    ///
    /// # Complexity
    /// - Time: O(k) on average, O(n) worst case
    /// - Space: O(k) for collected values plus O(h) recursion stack
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let k = k as usize;
        let mut values = Vec::with_capacity(k);
        Self::inorder(root, &mut values, k);
        values[k - 1]
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>, k: usize) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            Self::inorder(node.left.take(), values, k);
            if values.len() >= k {
                return;
            }
            values.push(node.val);
            if values.len() >= k {
                return;
            }
            Self::inorder(node.right.take(), values, k);
        }
    }
}
