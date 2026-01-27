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
    /// BFS level-order traversal summing the deepest level.
    ///
    /// # Intuition
    /// By processing the tree level by level, the sum computed during the last
    /// level is exactly the sum of the deepest leaves.
    ///
    /// # Approach
    /// 1. Initialize a queue with the root
    /// 2. For each level, reset the sum and accumulate all node values
    /// 3. Enqueue children for the next level
    /// 4. After the loop, the sum holds the deepest level's total
    ///
    /// # Complexity
    /// - Time: O(n) visiting every node once
    /// - Space: O(w) where w is the maximum level width
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut sum = 0;

        while !queue.is_empty() {
            sum = 0;
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    let node = node.borrow();
                    sum += node.val;
                    if node.left.is_some() {
                        queue.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone());
                    }
                }
            }
        }

        sum
    }
}
