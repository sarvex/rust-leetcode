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
    /// Returns the right side view of a binary tree using BFS level-order traversal.
    ///
    /// # Intuition
    /// The rightmost node at each level is visible from the right side.
    /// BFS processes levels; the first node dequeued when processing right-to-left
    /// is the rightmost.
    ///
    /// # Approach
    /// 1. BFS with a queue, processing right children before left.
    /// 2. At each level, the first node in the queue is the rightmost.
    /// 3. Collect these values as the right side view.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(w) where w is the maximum tree width
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let level_size = queue.len();
            result.push(queue[0].as_ref().unwrap().borrow().val);
            for _ in 0..level_size {
                if let Some(Some(node)) = queue.pop_front() {
                    let mut node = node.borrow_mut();
                    if node.right.is_some() {
                        queue.push_back(node.right.take());
                    }
                    if node.left.is_some() {
                        queue.push_back(node.left.take());
                    }
                }
            }
        }
        result
    }
}
