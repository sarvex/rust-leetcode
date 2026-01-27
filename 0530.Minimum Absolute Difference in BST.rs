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
    /// Finds the minimum absolute difference between any two BST nodes via in-order traversal.
    ///
    /// # Intuition
    /// In-order traversal of a BST yields sorted values. The minimum difference
    /// must occur between consecutive elements in this sorted sequence.
    ///
    /// # Approach
    /// 1. Perform in-order DFS, tracking the previous value.
    /// 2. Update the minimum difference at each node.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const INF: i32 = 1 << 30;
        let mut min_diff = INF;
        let mut prev = -INF;

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, min_diff: &mut i32, prev: &mut i32) {
            if let Some(rc) = node {
                let inner = rc.borrow();
                dfs(&inner.left, min_diff, prev);
                *min_diff = (*min_diff).min(inner.val - *prev);
                *prev = inner.val;
                dfs(&inner.right, min_diff, prev);
            }
        }

        dfs(&root, &mut min_diff, &mut prev);
        min_diff
    }
}
