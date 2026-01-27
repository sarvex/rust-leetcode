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
    /// BFS with alternating direction for zigzag level-order traversal.
    ///
    /// # Intuition
    /// Standard level-order BFS with a flag that toggles each level.
    /// On odd levels, reverse the collected values before appending.
    ///
    /// # Approach
    /// Process levels as in normal BFS. Toggle a `left_to_right` flag
    /// after each level. When the flag is false, reverse the level's
    /// values before adding to the result.
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once
    /// - Space: O(n) — queue may hold an entire level
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut left_to_right = true;

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

                if !left_to_right {
                    level.reverse();
                }
                result.push(level);
                left_to_right = !left_to_right;
            }
        }

        result
    }
}
