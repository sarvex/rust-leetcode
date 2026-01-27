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
    /// Computes the average value of each tree level using BFS.
    ///
    /// # Intuition
    /// Level-order traversal sums each level's values and divides by the
    /// level size for the average.
    ///
    /// # Approach
    /// 1. BFS with level-size counting.
    /// 2. Sum values using i64 to avoid overflow.
    /// 3. Compute the average for each level.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(w) where w is the maximum level width
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        while !queue.is_empty() {
            let level_size = queue.len();
            let mut sum: i64 = 0;
            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let inner = node.borrow();
                    sum += inner.val as i64;
                    if let Some(left) = inner.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = inner.right.clone() {
                        queue.push_back(right);
                    }
                }
            }
            result.push(sum as f64 / level_size as f64);
        }
        result
    }
}
