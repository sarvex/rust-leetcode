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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    /// BFS with final reversal for bottom-up level-order traversal.
    ///
    /// # Intuition
    /// Standard level-order BFS produces top-down results. Reversing the
    /// output yields bottom-up order without changing the traversal logic.
    ///
    /// # Approach
    /// Perform standard BFS collecting levels top-down. Reverse the result
    /// vector before returning.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once plus O(L) reversal
    /// - Space: O(n) — queue and result storage
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if let Some(root_node) = root {
            let mut queue = VecDeque::new();
            queue.push_back(root_node);

            while !queue.is_empty() {
                let level_size = queue.len();
                let mut level = Vec::with_capacity(level_size);

                for _ in 0..level_size {
                    if let Some(node) = queue.pop_front() {
                        let node = node.borrow();
                        level.push(node.val);
                        if let Some(ref left) = node.left {
                            queue.push_back(Rc::clone(left));
                        }
                        if let Some(ref right) = node.right {
                            queue.push_back(Rc::clone(right));
                        }
                    }
                }

                result.push(level);
            }
        }

        result.reverse();
        result
    }
}
