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
    /// Recursive structural and value comparison for tree equality.
    ///
    /// # Intuition
    /// Two trees are identical when both are empty, or both have the same
    /// root value with recursively identical left and right subtrees.
    ///
    /// # Approach
    /// Pattern match on both nodes. If both present, compare values and
    /// recurse on children. If both absent, return true. Mixed cases return false.
    ///
    /// # Complexity
    /// - Time: O(min(n, m)) — visits each node until a mismatch
    /// - Space: O(min(h1, h2)) — recursion depth is the shorter tree height
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dfs(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    let (a, b) = (a.borrow(), b.borrow());
                    a.val == b.val && dfs(&a.left, &b.left) && dfs(&a.right, &b.right)
                }
                _ => false,
            }
        }

        dfs(&p, &q)
    }
}
