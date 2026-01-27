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
    /// Returns preorder traversal of a binary tree using recursive DFS.
    ///
    /// # Intuition
    /// Preorder visits root, then left subtree, then right subtree.
    ///
    /// # Approach
    /// Recursively collect node values in root-left-right order.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(h) recursion stack where h is tree height
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::dfs(&root, &mut result);
        result
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            result.push(node.val);
            Self::dfs(&node.left, result);
            Self::dfs(&node.right, result);
        }
    }
}
