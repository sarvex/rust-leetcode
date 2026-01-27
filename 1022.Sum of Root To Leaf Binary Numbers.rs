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
    /// Sums all root-to-leaf binary numbers via DFS.
    ///
    /// # Intuition
    /// Each path from root to leaf represents a binary number built by
    /// shifting left and OR-ing the current node's bit.
    ///
    /// # Approach
    /// DFS passing the accumulated binary value. At each node, shift left
    /// and OR the value. At leaves, return the accumulated number.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, num: i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let val = (num << 1) | n.val;
                    if n.left.is_none() && n.right.is_none() {
                        val
                    } else {
                        dfs(&n.left, val) + dfs(&n.right, val)
                    }
                }
            }
        }
        dfs(&root, 0)
    }
}
