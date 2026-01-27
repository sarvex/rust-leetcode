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
    /// Merges two binary trees by summing overlapping nodes.
    ///
    /// # Intuition
    /// Recursively merge corresponding nodes. Where only one tree has a node,
    /// use that subtree directly.
    ///
    /// # Approach
    /// 1. Match on both roots.
    /// 2. If both present, sum values and recurse on children.
    /// 3. If only one present, return that subtree.
    ///
    /// # Complexity
    /// - Time: O(min(m, n)) where m, n are node counts
    /// - Space: O(min(h1, h2)) recursion depth
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (Some(r), None) | (None, Some(r)) => Some(r),
            (Some(r1), Some(r2)) => {
                {
                    let mut n1 = r1.borrow_mut();
                    let mut n2 = r2.borrow_mut();
                    n1.val += n2.val;
                    n1.left = Self::merge_trees(n1.left.take(), n2.left.take());
                    n1.right = Self::merge_trees(n1.right.take(), n2.right.take());
                }
                Some(r1)
            }
        }
    }
}
