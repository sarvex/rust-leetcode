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
    /// Checks if one tree is a subtree of another via recursive comparison.
    ///
    /// # Intuition
    /// A subtree match means there exists a node in the main tree where the
    /// entire sub-tree structure is identical. Check every node as a potential root.
    ///
    /// # Approach
    /// 1. For each node in the main tree, check if it matches the sub-tree root.
    /// 2. Matching requires identical values and identical children recursively.
    ///
    /// # Complexity
    /// - Time: O(m × n) worst case
    /// - Space: O(h) recursion depth
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn same(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    let a = a.borrow();
                    let b = b.borrow();
                    a.val == b.val && same(&a.left, &b.left) && same(&a.right, &b.right)
                }
                _ => false,
            }
        }

        match &root {
            None => false,
            Some(rc) => {
                let node = rc.borrow();
                same(&root, &sub_root)
                    || Self::is_subtree(node.left.clone(), sub_root.clone())
                    || Self::is_subtree(node.right.clone(), sub_root.clone())
            }
        }
    }
}
