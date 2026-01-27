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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    /// Finds the leftmost value in the bottom row using BFS.
    ///
    /// # Intuition
    /// Level-order traversal naturally visits nodes left to right. The first
    /// node of the last level is the bottom-left value.
    ///
    /// # Approach
    /// 1. BFS level by level.
    /// 2. Record the first value of each level.
    /// 3. The last recorded value is the answer.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(w) where w is the maximum level width
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut result = 0;
        while !queue.is_empty() {
            result = queue.front().unwrap().as_ref().unwrap().borrow().val;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let inner = node.as_ref().unwrap().borrow();
                if inner.left.is_some() {
                    queue.push_back(inner.left.clone());
                }
                if inner.right.is_some() {
                    queue.push_back(inner.right.clone());
                }
            }
        }
        result
    }
}
