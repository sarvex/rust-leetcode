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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    /// Finds the most frequent subtree sums via DFS.
    ///
    /// # Intuition
    /// Compute the subtree sum at each node recursively, count frequencies,
    /// and return all sums matching the maximum frequency.
    ///
    /// # Approach
    /// 1. DFS computes each subtree sum and records it in a frequency map.
    /// 2. Find the maximum frequency.
    /// 3. Collect all sums with that frequency.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, freq: &mut HashMap<i32, i32>) -> i32 {
            match node {
                None => 0,
                Some(rc) => {
                    let inner = rc.borrow();
                    let sum = inner.val + dfs(&inner.left, freq) + dfs(&inner.right, freq);
                    *freq.entry(sum).or_insert(0) += 1;
                    sum
                }
            }
        }

        let mut freq = HashMap::new();
        dfs(&root, &mut freq);
        let max_count = freq.values().copied().max().unwrap_or(0);
        freq.into_iter()
            .filter(|&(_, v)| v == max_count)
            .map(|(k, _)| k)
            .collect()
    }
}
