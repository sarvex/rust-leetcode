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
use std::rc::Rc;

impl Solution {
    /// Recursive enumeration of all structurally unique BSTs for values [1..n].
    ///
    /// # Intuition
    /// Each value in the range can serve as the root. Left and right subtrees
    /// are recursively built from the values below and above the root. The
    /// cartesian product of left and right subtrees gives all unique trees.
    ///
    /// # Approach
    /// For range `[i, j]`, iterate each value `v` as root. Recursively
    /// generate all left subtrees from `[i, v-1]` and right subtrees from
    /// `[v+1, j]`. Combine every left-right pair with root `v`.
    ///
    /// # Complexity
    /// - Time: O(n × C(n)) — Catalan number of trees, each with n nodes
    /// - Space: O(n × C(n)) — storing all trees
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(lo: i32, hi: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if lo > hi {
                return vec![None];
            }
            let mut trees = Vec::new();
            for root_val in lo..=hi {
                let left_trees = dfs(lo, root_val - 1);
                let right_trees = dfs(root_val + 1, hi);
                for left in &left_trees {
                    for right in &right_trees {
                        trees.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: root_val,
                            left: left.clone(),
                            right: right.clone(),
                        }))));
                    }
                }
            }
            trees
        }

        dfs(1, n)
    }
}
