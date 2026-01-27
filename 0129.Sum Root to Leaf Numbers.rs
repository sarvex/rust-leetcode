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
    /// Sums all root-to-leaf numbers using DFS with accumulated value.
    ///
    /// # Intuition
    /// Each path from root to leaf forms a number by appending digits.
    /// Accumulate the number by multiplying by 10 at each level and adding
    /// the current node's value.
    ///
    /// # Approach
    /// 1. DFS with a running accumulated number.
    /// 2. At each node, compute `num * 10 + val`.
    /// 3. At leaf nodes, return the accumulated number.
    /// 4. For internal nodes, return the sum of left and right subtree results.
    ///
    /// # Complexity
    /// - Time: O(n) — visit every node once
    /// - Space: O(h) — recursion stack where h is tree height
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 0)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, num: i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                let current = num * 10 + n.val;
                if n.left.is_none() && n.right.is_none() {
                    current
                } else {
                    Self::dfs(&n.left, current) + Self::dfs(&n.right, current)
                }
            }
        }
    }
}
