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
    /// Finds minimum difference between any two BST nodes via in-order traversal.
    ///
    /// # Intuition
    /// In-order traversal of a BST yields sorted values. The minimum difference
    /// must occur between consecutive elements in this sorted sequence.
    ///
    /// # Approach
    /// Perform in-order DFS, tracking the previous value. At each node, update
    /// the minimum difference as `current - previous`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) where h is tree height for recursion stack
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const INF: i32 = 1 << 30;
        let mut ans = INF;
        let mut prev = -INF;

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, prev: &mut i32) {
            if let Some(n) = node {
                let n = n.borrow();
                dfs(n.left.clone(), ans, prev);
                *ans = (*ans).min(n.val - *prev);
                *prev = n.val;
                dfs(n.right.clone(), ans, prev);
            }
        }

        dfs(root, &mut ans, &mut prev);
        ans
    }
}
