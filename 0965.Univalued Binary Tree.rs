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
    /// Checks if all nodes in a binary tree have the same value.
    ///
    /// # Intuition
    /// DFS comparing every node's value to the root's value.
    ///
    /// # Approach
    /// Extract the root value. Recursively verify that every node matches.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let target = root.as_ref().unwrap().borrow().val;

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
            match node {
                None => true,
                Some(n) => {
                    let n = n.borrow();
                    n.val == val && dfs(&n.left, val) && dfs(&n.right, val)
                }
            }
        }

        dfs(&root, target)
    }
}
