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
    /// Finds the largest value in each tree row using BFS.
    ///
    /// # Intuition
    /// Level-order traversal processes one row at a time, making it natural
    /// to track the maximum value per level.
    ///
    /// # Approach
    /// 1. BFS with level-size counting.
    /// 2. Track the maximum value within each level.
    /// 3. Collect maximums into the result.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(w) where w is the maximum level width
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            let mut level_max = i32::MIN;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let inner = node.as_ref().unwrap().borrow();
                level_max = level_max.max(inner.val);
                if inner.left.is_some() {
                    queue.push_back(inner.left.clone());
                }
                if inner.right.is_some() {
                    queue.push_back(inner.right.clone());
                }
            }
            result.push(level_max);
        }
        result
    }
}
