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
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

impl Solution {
    /// Checks if two nodes in a BST sum to k using BFS with a complement set.
    ///
    /// # Intuition
    /// For each node, check if its complement (k - val) has been seen.
    /// BFS traversal visits all nodes systematically.
    ///
    /// # Approach
    /// 1. BFS through the tree.
    /// 2. For each node, check if its value is in the complement set.
    /// 3. Add (k - val) to the set.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            if let Some(rc) = node {
                let inner = rc.borrow();
                if seen.contains(&inner.val) {
                    return true;
                }
                seen.insert(k - inner.val);
                queue.push_back(inner.left.clone());
                queue.push_back(inner.right.clone());
            }
        }
        false
    }
}
