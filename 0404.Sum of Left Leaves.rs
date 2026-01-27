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
    /// Sums all left leaves in a binary tree via recursive DFS.
    ///
    /// # Intuition
    /// A left leaf is a node with no children that is the left child of its
    /// parent. Track the "is left child" flag through recursion.
    ///
    /// # Approach
    /// 1. DFS with an `is_left` flag.
    /// 2. If the node is a leaf and `is_left`, return its value.
    /// 3. Otherwise recurse on both children, passing the appropriate flag.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion depth
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            match node {
                None => 0,
                Some(rc) => {
                    let inner = rc.borrow();
                    if inner.left.is_none() && inner.right.is_none() {
                        return if is_left { inner.val } else { 0 };
                    }
                    dfs(&inner.left, true) + dfs(&inner.right, false)
                }
            }
        }
        dfs(&root, false)
    }
}
