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
    /// Finds the lowest common ancestor of two nodes using recursive DFS.
    ///
    /// # Intuition
    /// If the current node is p or q, it is part of the answer. If both
    /// subtrees return non-None, the current node is the LCA.
    ///
    /// # Approach
    /// 1. Base case: return root if it is None, p, or q.
    /// 2. Recurse into left and right subtrees.
    /// 3. If both return a node, root is the LCA.
    /// 4. Otherwise, return whichever subtree found a match.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }
        let node = root.as_ref().unwrap().borrow();
        let left = Self::lowest_common_ancestor(node.left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(node.right.clone(), p, q);
        match (&left, &right) {
            (Some(_), Some(_)) => {
                drop(node);
                root
            }
            (None, _) => right,
            _ => left,
        }
    }
}
