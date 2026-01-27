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
    /// Reverses node values at every odd level of a perfect binary tree.
    ///
    /// # Intuition
    /// Perform a level-order traversal collecting values per level, reverse
    /// odd-level values, then reconstruct the tree from the collected levels.
    ///
    /// # Approach
    /// 1. BFS to collect values level by level, reversing odd levels
    /// 2. Reconstruct the tree recursively using the collected level values
    ///
    /// # Complexity
    /// - Time: O(n) — each node visited once during BFS and once during reconstruction
    /// - Space: O(n) — storage for all node values
    fn create_tree(vals: &[Vec<i32>], i: usize, j: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if i >= vals.len() {
            return None;
        }
        Some(Rc::new(RefCell::new(TreeNode {
            val: vals[i][j],
            left: Self::create_tree(vals, i + 1, j * 2),
            right: Self::create_tree(vals, i + 1, j * 2 + 1),
        })))
    }

    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut depth = 0;
        let mut vals = Vec::new();

        while !queue.is_empty() {
            let mut level_vals = Vec::with_capacity(queue.len());
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let mut node = node.as_ref().unwrap().borrow_mut();
                level_vals.push(node.val);
                if node.left.is_some() {
                    queue.push_back(node.left.take());
                }
                if node.right.is_some() {
                    queue.push_back(node.right.take());
                }
            }
            if depth % 2 == 1 {
                level_vals.reverse();
            }
            vals.push(level_vals);
            depth += 1;
        }

        Self::create_tree(&vals, 0, 0)
    }
}
